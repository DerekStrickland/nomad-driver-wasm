mod proto;

use env_logger;
use log;
use std::str;
use std::time::{Duration};

use crate::proto::base::{PluginInfoResponse, PluginType};
use crate::proto::drivers::{DriverCapabilities};
use crate::proto::drivers::network_isolation_spec::{NetworkIsolationMode};
use crate::proto::hclext;
use crate::proto::hclspec::{Default, Spec, Literal};
use std::collections::HashMap;

// PLUGIN_NAME is the name of the plugin
// this is used for logging and (along with the version) for uniquely
// identifying plugin binaries fingerprinted by the client
const PLUGIN_NAME: &str = "nomad-driver-wasmtime";

// // PLUGIN_VERSION allows the client to identify and use newer versions of
// // an installed plugin
const PLUGIN_VERSION: &str = "v0.0.1";

// // FINGERPRINT_PERIOD is the interval at which the plugin will send
// // fingerprint responses
const FINGERPRINT_PERIOD: Duration = Duration::from_secs(30);

// // TASK_HANDLE_VERSION is the version of task handle which this plugin sets
// // and understands how to decode
// // this is used to allow modification and migration of the task schema
// // used by the plugin
const TASK_HANDLE_VERSION: u8 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting nomad-driver-wasmtime server");
    log::info!("Plugin Name: {}", PLUGIN_NAME);
    log::info!("Plugin Version: {}", PLUGIN_VERSION);
    log::info!("Fingerprint Period: {}s", FINGERPRINT_PERIOD.as_secs());
    log::info!("Task Handle Version: {}", TASK_HANDLE_VERSION);

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

// pluginInfo describes the plugin
static pluginInfo: PluginInfoResponse = PluginInfoResponse{
    r#type: PluginType::Driver as i32,
    plugin_api_versions: vec!["v0.1.0".to_string()],
    plugin_version: PLUGIN_VERSION.to_string(),
    name: PLUGIN_NAME.to_string()
};

// configSpec is the specification of the plugin's configuration
// this is used to validate the configuration specified for the plugin
// on the client.
// this is not global, but can be specified on a per-client basis.


static configSpec: HashMap<&str, Spec> = buildConfigSpec();

fn buildConfigSpec() -> HashMap<&str, Spec> {
    let mut attrs: HashMap<&str, Spec> = HashMap::new();

    attrs.insert(
        "enabled",
        hclext::default_spec(
            Default{
                    primary: hclext::new_attr(String::from("enabled"), "bool", false),
                    default:  Some(Box::from(hclext::new_literal(Literal{
                        value: "true".to_string()
                    })))
                }
        ));



    attrs.insert("wasmtime_runtime",
                 hclext::new_attr_spec(
                    String::from("wasmtime_runtime"),
                    String::from("string"),
                    true
                    )
    );

    attrs.insert("stats_interval",
                 hclext::new_attr_spec(
                     String::from("stats_interval"),
                     String::from("string"),
                        false
                 )
    );

    attrs.insert("allow_privileged",
                 hclext::new_default_spec(
                hclext::new_attr_spec(
                    String::from("allow_privileged"),
                    String::from("bool"),
                false),
            hclext::new_literal_spec(
                String::from("true")
                    )
               )
        );


    let mut auth_map:HashMap<String, Spec> = HashMap::new();
    auth_map.insert(
        String::from("username"),
        hclext::new_attr_spec(
            String::from("username"),
            String::from("string"),
            true
        )
    );

    auth_map.insert(
        String::from("password"),
        hclext::new_attr_spec(String::from(), String::from("string"), true)
    );

    attrs.insert(
        "auth",
        hclext::new_object_spec(auth_map)
    );

    attrs
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
