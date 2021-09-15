mod plugin;
mod proto;

use env_logger;
use log;

use crate::plugin::{Plugin};
use crate::proto::drivers::{DriverCapabilities};
use crate::proto::drivers::network_isolation_spec::{NetworkIsolationMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Initialize the plugin
    let plugin = Plugin::new();
    log::info!("Plugin: {:?}", plugin);

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

    log::info!("Driver Capabilities: {}", driver_capabilities.exec);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{main};
    use tokio_test::{assert_ok};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_main() {
        assert_ok!(main())
    }
}
