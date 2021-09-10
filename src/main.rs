mod proto;

use env_logger;
use log;
use std::str;
use std::time::{Duration};

use crate::proto::drivers::{DriverCapabilities};
use crate::proto::drivers::network_isolation_spec::{NetworkIsolationMode};

// PLUGIN_NAME is the name of the plugin
// this is used for logging and (along with the version) for uniquely
// identifying plugin binaries fingerprinted by the client
const PLUGIN_NAME: &str = "nomad-driver-wasmtime";

// // PLUGIN_VERSION allows the client to identify and use newer versions of
// // an installed plugin
const PLUGIN_VERSION: &str = "v0.0.1";

// // FINGERPRINT_PERIOD is the interval at which the plugin will send
// // fingerprint responses
const FINGERPRINT_PERIOD: Duration = Duration::new(30, 0);

// // TASK_HANDLE_VERSION is the version of task handle which this plugin sets
// // and understands how to decode
// // this is used to allow modification and migration of the task schema
// // used by the plugin
const TASK_HANDLE_VERSION: u8 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting nomad-driver-wasmtime server");

    let driver_capabilities = DriverCapabilities{
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
    };

    log::info!("{}", driver_capabilities.exec);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
