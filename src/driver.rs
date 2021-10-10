#![allow(unused_variables)]
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tonic::{Request, Response, Status, Streaming};

// use log;
// use log::LevelFilter;
// use log4rs::append::file::FileAppender;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;
use rmp_serde;

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::base::proto as base;
use nomad::drivers::proto as drivers;
use nomad::shared::hclspec;
use nomad::shared::structs;

use crate::fingerprint::fingerprinter::build_fingerprint_attrs;
use crate::hclext;

use base::base_plugin_server::BasePlugin;
use base::{
    ConfigSchemaRequest, ConfigSchemaResponse, NomadConfig, PluginInfoRequest, PluginInfoResponse,
    PluginType, SetConfigRequest, SetConfigResponse,
};

use drivers::driver_server::Driver;
use drivers::fingerprint_response::HealthState;
use drivers::network_isolation_spec::NetworkIsolationMode;
use drivers::{
    CapabilitiesRequest, CapabilitiesResponse, CreateNetworkRequest, CreateNetworkResponse,
    DestroyNetworkRequest, DestroyNetworkResponse, DestroyTaskRequest, DestroyTaskResponse,
    DriverCapabilities, DriverTaskEvent, ExecTaskRequest, ExecTaskResponse,
    ExecTaskStreamingRequest, ExecTaskStreamingResponse, FingerprintRequest, FingerprintResponse,
    InspectTaskRequest, InspectTaskResponse, RecoverTaskRequest, RecoverTaskResponse,
    SignalTaskRequest, SignalTaskResponse, StartTaskRequest, StartTaskResponse, StopTaskRequest,
    StopTaskResponse, TaskConfigSchemaRequest, TaskConfigSchemaResponse, TaskEventsRequest,
    TaskStatsRequest, TaskStatsResponse, WaitTaskRequest, WaitTaskResponse,
};
use hclspec::{Default, Spec};
use structs::attribute::Value;

/// WasmDriver is the Nomad TaskDriver implementation for running wasm tasks.
pub struct WasmDriver {
    /// config_spec is the specification of the plugin's configuration
    /// this is used to validate the configuration specified for the plugin
    /// on the client. This is not global, but can be specified on a per-client basis.
    config_schema: Arc<Mutex<Spec>>,
    /// capabilities returns the features or capabilities that the plugin provides.
    capabilities: DriverCapabilities,
    /// nomad_config is the client config from Nomad
    nomad_config: Arc<Mutex<NomadConfig>>,
    // TODO: Can I safely remove this? Looks like I just need to set this value
    // on the PluginInfoResponse from the constant. Possibly this might be used
    // to see if Nomad's plugin api version has changed?
    plugin_api_version: Arc<Mutex<String>>,
    /// plugin_info returns the configuration for the plugin, which will be requested
    /// by Nomad during at least plugin loading.
    plugin_info: PluginInfoResponse,
}

impl core::default::Default for WasmDriver {
    fn default() -> Self {
        WasmDriver {
            config_schema: Arc::new(Mutex::new(WasmDriver::default_config_spec())),
            capabilities: WasmDriver::default_capabilities(),
            nomad_config: Arc::new(Mutex::new(NomadConfig { driver: None })),
            plugin_api_version: Arc::new(Mutex::new(String::from(API_VERSION))),
            plugin_info: WasmDriver::default_plugin_info(),
        }
    }
}

#[tonic::async_trait]
impl BasePlugin for WasmDriver {
    async fn plugin_info(
        &self,
        request: Request<PluginInfoRequest>,
    ) -> Result<Response<PluginInfoResponse>, Status> {
        // log::info!("Received PluginInfoRequest");
        Ok(tonic::Response::new(self.plugin_info.clone()))
    }

    async fn config_schema(
        &self,
        request: Request<ConfigSchemaRequest>,
    ) -> Result<Response<ConfigSchemaResponse>, Status> {
        // log::info!("Received ConfigSchemaRequest");
        Ok(tonic::Response::new(ConfigSchemaResponse {
            spec: Some(self.config_schema.lock().unwrap().deref().clone()),
        }))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        // log::info!("Received SetConfigRequest");

        let request_ref = request.get_ref();

        if request_ref.msgpack_config.is_empty() {
            // log::error!("msgpack_config is required");
            return Err(Status::invalid_argument("msgpack_config"));
        }

        if request_ref.plugin_api_version.is_empty() {
            // log::error!("plugin_api_version is required");
            return Err(Status::invalid_argument("plugin_api_version"));
        }

        let config_schema = Arc::clone(&self.config_schema);
        let mut cs = config_schema.lock().unwrap();

        *cs = rmp_serde::from_slice(request_ref.msgpack_config.as_slice())
            .or_else(|e| Err(Status::invalid_argument("msgpack_config")))?;

        let nomad_config = Arc::clone(&self.nomad_config);
        let mut nc = nomad_config.lock().unwrap();

        match request_ref.clone().nomad_config {
            Some(c) => *nc = c,
            None => log::error!("nomad_config is required but passed guard clause"),
        }

        let plugin_api_version = Arc::clone(&self.plugin_api_version);
        let mut pav = plugin_api_version.lock().unwrap();
        *pav = request_ref.clone().plugin_api_version;

        Ok(tonic::Response::new(SetConfigResponse {}))
    }
}

#[tonic::async_trait]
impl Driver for WasmDriver {
    async fn task_config_schema(
        &self,
        request: Request<TaskConfigSchemaRequest>,
    ) -> Result<Response<TaskConfigSchemaResponse>, Status> {
        // log::info!("Received TaskConfigSchemaRequest");
        Ok(tonic::Response::new(TaskConfigSchemaResponse {
            spec: None,
        }))
    }

    async fn capabilities(
        &self,
        request: Request<CapabilitiesRequest>,
    ) -> Result<Response<CapabilitiesResponse>, Status> {
        // log::info!("Received CapabilitiesRequest");
        Ok(tonic::Response::new(CapabilitiesResponse {
            capabilities: Some(WasmDriver::default_capabilities()),
        }))
    }

    type FingerprintStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<FingerprintResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn fingerprint(
        &self,
        request: Request<FingerprintRequest>,
    ) -> Result<Response<Self::FingerprintStream>, Status> {
        // log::info!("Received FingerprintRequest");

        let (tx, rx) = tokio::sync::mpsc::channel(4);

        let attrs = build_fingerprint_attrs();

        let fingerprint_response = FingerprintResponse {
            attributes: attrs.clone(),
            health: HealthState::Healthy as i32,
            health_description: String::from("healthy"),
        };

        for (k, v) in attrs {
            match v.value {
                Some(Value::StringVal(val)) => {
                    log::info!("attribute {}: {}", k, val)
                }
                _ => log::info!("attribute {} is not a string", k),
            }
        }

        // log::info!("health {}", fingerprint_response.health);
        // log::info!(
        //     "health_description {}",
        //     fingerprint_response.health_description
        // );

        tokio::spawn(async move {
            loop {
                tx.send(Ok(fingerprint_response.clone())).await.unwrap();
                tokio::time::sleep(FINGERPRINT_PERIOD).await;
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn recover_task(
        &self,
        request: Request<RecoverTaskRequest>,
    ) -> Result<Response<RecoverTaskResponse>, Status> {
        // log::info!("Received RecoverTaskRequest");
        Ok(tonic::Response::new(RecoverTaskResponse {}))
    }

    async fn start_task(
        &self,
        request: Request<StartTaskRequest>,
    ) -> Result<Response<StartTaskResponse>, Status> {
        // log::info!("Received StartTaskRequest");
        Ok(tonic::Response::new(StartTaskResponse {
            result: 0,
            driver_error_msg: "".to_string(),
            handle: None,
            network_override: None,
        }))
    }

    async fn wait_task(
        &self,
        request: Request<WaitTaskRequest>,
    ) -> Result<Response<WaitTaskResponse>, Status> {
        // log::info!("Received WaitTaskRequest");
        Ok(tonic::Response::new(WaitTaskResponse {
            result: None,
            err: "".to_string(),
        }))
    }

    async fn stop_task(
        &self,
        request: Request<StopTaskRequest>,
    ) -> Result<Response<StopTaskResponse>, Status> {
        // log::info!("Received StopTaskRequest");
        Ok(tonic::Response::new(StopTaskResponse {}))
    }

    async fn destroy_task(
        &self,
        request: Request<DestroyTaskRequest>,
    ) -> Result<Response<DestroyTaskResponse>, Status> {
        // log::info!("Received DestroyTaskRequest");
        Ok(tonic::Response::new(DestroyTaskResponse {}))
    }

    async fn inspect_task(
        &self,
        request: Request<InspectTaskRequest>,
    ) -> Result<Response<InspectTaskResponse>, Status> {
        // log::info!("Received InspectTaskRequest");
        Ok(tonic::Response::new(InspectTaskResponse {
            task: None,
            driver: None,
            network_override: None,
        }))
    }

    type TaskStatsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<TaskStatsResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn task_stats(
        &self,
        request: Request<TaskStatsRequest>,
    ) -> Result<Response<Self::TaskStatsStream>, Status> {
        // log::info!("Received TaskStatsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    type TaskEventsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<DriverTaskEvent, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn task_events(
        &self,
        request: Request<TaskEventsRequest>,
    ) -> Result<Response<Self::TaskEventsStream>, Status> {
        // log::info!("Received TaskEventsRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn signal_task(
        &self,
        request: Request<SignalTaskRequest>,
    ) -> Result<Response<SignalTaskResponse>, Status> {
        // log::info!("Received SignalTaskRequest");
        Ok(tonic::Response::new(SignalTaskResponse {}))
    }

    async fn exec_task(
        &self,
        request: Request<ExecTaskRequest>,
    ) -> Result<Response<ExecTaskResponse>, Status> {
        // log::info!("Received ExecTaskRequest");
        Ok(tonic::Response::new(ExecTaskResponse {
            stdout: vec![],
            stderr: vec![],
            result: None,
        }))
    }

    type ExecTaskStreamingStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<ExecTaskStreamingResponse, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn exec_task_streaming(
        &self,
        request: Request<Streaming<ExecTaskStreamingRequest>>,
    ) -> Result<Response<Self::ExecTaskStreamingStream>, Status> {
        // log::info!("Received ExecTaskStreamingRequest");
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(receiver),
        )))
    }

    async fn create_network(
        &self,
        request: Request<CreateNetworkRequest>,
    ) -> Result<Response<CreateNetworkResponse>, Status> {
        // log::info!("Received CreateNetworkRequest");
        Ok(tonic::Response::new(CreateNetworkResponse {
            isolation_spec: None,
            created: false,
        }))
    }

    async fn destroy_network(
        &self,
        request: Request<DestroyNetworkRequest>,
    ) -> Result<Response<DestroyNetworkResponse>, Status> {
        // log::info!("Received DestroyNetworkRequest");
        Ok(tonic::Response::new(DestroyNetworkResponse {}))
    }
}

impl WasmDriver {
    fn default_plugin_info() -> PluginInfoResponse {
        PluginInfoResponse {
            r#type: PluginType::Driver as i32,
            plugin_api_versions: vec![String::from(API_VERSION)],
            plugin_version: String::from(PLUGIN_VERSION),
            name: String::from(PLUGIN_NAME),
        }
    }

    fn default_config_spec() -> Spec {
        let mut attrs: HashMap<String, Spec> = HashMap::new();

        // Flag for managing task driver enabled status.
        attrs.insert(
            String::from("enabled"),
            hclext::default_spec(Default {
                primary: Some(Box::from(hclext::new_attr_spec(
                    String::from("enabled"),
                    String::from("bool"),
                    false,
                ))),
                default: Some(Box::from(hclext::new_literal_spec(String::from("true")))),
            }),
        );

        // Wasmtime runtime executable path.
        attrs.insert(
            String::from("wasm_runtime"),
            hclext::new_attr_spec(String::from("wasm_runtime"), String::from("string"), true),
        );

        // Interval for collections TaskStats.
        attrs.insert(
            String::from("stats_interval"),
            hclext::new_attr_spec(
                String::from("stats_interval"),
                String::from("string"),
                false,
            ),
        );

        // If set to false, the driver will deny running privileged jobs.
        attrs.insert(
            String::from("allow_privileged"),
            hclext::new_default_spec(
                hclext::new_attr_spec(
                    String::from("allow_privileged"),
                    String::from("bool"),
                    false,
                ),
                hclext::new_literal_spec(String::from("true")),
            ),
        );

        // Provide authentication for a private registry.
        let mut auth_map: HashMap<String, Spec> = HashMap::new();
        auth_map.insert(
            String::from("username"),
            hclext::new_attr_spec(String::from("username"), String::from("string"), true),
        );

        auth_map.insert(
            String::from("password"),
            hclext::new_attr_spec(String::from("password"), String::from("string"), true),
        );

        let auth_block = hclext::new_block_spec(
            String::from("auth"),
            false,
            hclext::new_object_spec(auth_map),
        );

        attrs.insert(String::from("auth"), auth_block);

        hclext::new_object_spec(attrs)
    }

    fn default_capabilities() -> DriverCapabilities {
        DriverCapabilities {
            send_signals: true,
            exec: true,
            fs_isolation: 0,
            network_isolation_modes: vec![
                NetworkIsolationMode::Host as i32,
                NetworkIsolationMode::Group as i32,
                NetworkIsolationMode::Task as i32,
                NetworkIsolationMode::None as i32,
            ],
            must_create_network: false,
            mount_configs: 0,
            remote_tasks: false,
        }
    }
}

/// PLUGIN_NAME is the name of the plugin. This is used for logging and (along
/// with the version) for uniquely identifying plugin binaries fingerprinted by
/// the client.
pub const PLUGIN_NAME: &str = "nomad-driver-wasm";

/// PLUGIN_VERSION allows the client to identify and use newer versions of
/// an installed plugin.
pub const PLUGIN_VERSION: &str = "v0.1.0";

/// FINGERPRINT_PERIOD is the interval at which the plugin will send fingerprint
/// responses.
pub const FINGERPRINT_PERIOD: Duration = Duration::from_secs(30);

/// TASK_HANDLE_VERSION is the version of task handle which this plugin sets
/// and understands how to decode. This is used to allow modification and migration
/// of the task schema used by the plugin.
pub const TASK_HANDLE_VERSION: u8 = 1;

/// API_VERSION must match the version from the nomad/drivers/versions.go file.
const API_VERSION: &str = "v0.1.0";
