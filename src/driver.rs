#![allow(unused_variables)]
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tonic::{Request, Response, Status, Streaming};

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::base::proto as base;
use nomad::drivers::proto as drivers;
use nomad::shared::hclspec;

use crate::hclext;

use base::{NomadConfig, PluginInfoResponse, PluginType};

use crate::task::TaskController;
use drivers::network_isolation_spec::NetworkIsolationMode;
use drivers::DriverCapabilities;
use hclspec::{Default, Spec};

mod config;
mod driver;
mod plugin;

/// WasmDriver is the Nomad TaskDriver implementation for running wasm tasks.
pub struct WasmDriver {
    // Task
    controller: Arc<Mutex<TaskController>>,
    /// config_spec is the specification of the plugin's configuration
    /// this is used to validate the configuration specified for the plugin
    /// on the client. This is not global, but can be specified on a per-client basis.
    config_schema: Arc<Mutex<Spec>>,
    /// capabilities returns the features or capabilities that the plugin provides.
    capabilities: DriverCapabilities,
    /// nomad_config is the client config from Nomad
    nomad_config: Arc<Mutex<NomadConfig>>,
    /// plugin_info returns the configuration for the plugin, which will be requested
    /// by Nomad during at least plugin loading.
    plugin_info: PluginInfoResponse,
}

impl core::default::Default for WasmDriver {
    fn default() -> Self {
        WasmDriver {
            controller: Arc::new(Mutex::new(TaskController::default())),
            config_schema: Arc::new(Mutex::new(WasmDriver::default_config_spec())),
            capabilities: WasmDriver::default_capabilities(),
            nomad_config: Arc::new(Mutex::new(NomadConfig { driver: None })),
            plugin_info: WasmDriver::default_plugin_info(),
        }
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
        let mut config_spec: HashMap<String, Spec> = HashMap::new();

        // Flag for managing task driver enabled status.
        config_spec.insert(
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
        config_spec.insert(
            String::from("wasm_runtime"),
            hclext::new_attr_spec(String::from("wasm_runtime"), String::from("string"), true),
        );

        // Interval for collections TaskStats.
        config_spec.insert(
            String::from("stats_interval"),
            hclext::new_attr_spec(
                String::from("stats_interval"),
                String::from("string"),
                false,
            ),
        );

        // If set to false, the driver will deny running privileged jobs.
        config_spec.insert(
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

        config_spec.insert(String::from("auth"), auth_block);

        hclext::new_object_spec(config_spec)
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
