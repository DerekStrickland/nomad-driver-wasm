mod plugin;
mod proto;

use env_logger;
use log;

use crate::proto::drivers::{DriverCapabilities};
use crate::proto::drivers::network_isolation_spec::{NetworkIsolationMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting nomad-driver-wasmtime server");
    log::info!("Plugin Name: {}", plugin::PLUGIN_NAME );
    log::info!("Plugin Version: {}", plugin::PLUGIN_VERSION);
    log::info!("Fingerprint Period: {}s", plugin::FINGERPRINT_PERIOD.as_secs());
    log::info!("Task Handle Version: {}", plugin::TASK_HANDLE_VERSION);

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
    use prost;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn check_scalar_types() {
        prost::tests::check_message(&ScalarTypes::default());
    }
}
