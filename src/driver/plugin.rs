use super::{Arc, Deref, Request, Response, Status, WasmDriver};

use rmp_serde;

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::base::proto as base;

use base::base_plugin_server::BasePlugin;
use base::{
    ConfigSchemaRequest, ConfigSchemaResponse, PluginInfoRequest, PluginInfoResponse,
    SetConfigRequest, SetConfigResponse,
};

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

        Ok(tonic::Response::new(SetConfigResponse {}))
    }
}
