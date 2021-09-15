use std::collections::HashMap;
use std::time::{Duration};

use crate::proto::base::{PluginInfoResponse, PluginType};
use crate::proto::hclext;
use crate::proto::hclspec::{Default, Spec};

// PLUGIN_NAME is the name of the plugin
// this is used for logging and (along with the version) for uniquely
// identifying plugin binaries fingerprinted by the client
pub const PLUGIN_NAME: &str = "nomad-driver-wasmtime";

// // PLUGIN_VERSION allows the client to identify and use newer versions of
// // an installed plugin
pub const PLUGIN_VERSION: &str = "v0.0.1";

// // FINGERPRINT_PERIOD is the interval at which the plugin will send
// // fingerprint responses
pub const FINGERPRINT_PERIOD: Duration = Duration::from_secs(30);

// // TASK_HANDLE_VERSION is the version of task handle which this plugin sets
// // and understands how to decode
// // this is used to allow modification and migration of the task schema
// // used by the plugin
pub const TASK_HANDLE_VERSION: u8 = 1;

// API_VERSION is the version from the .proto file
const API_VERSION: &str = "v0.1.0";

// Plugin encapsulates all the runtime configuration required to interact with
// the Nomad plugin host.
#[derive(Debug)]
pub struct Plugin {
    // plugin_info returns the configuration for the plugin, which will be requested
    // by Nomad during at least plugin loading.
    pub plugin_info: PluginInfoResponse,
    // config_spec is the specification of the plugin's configuration
    // this is used to validate the configuration specified for the plugin
    // on the client. This is not global, but can be specified on a per-client basis.
    pub config_spec: HashMap<&'static str, Spec>,
}

impl Plugin {
    pub fn new() -> Plugin {
        Plugin{
            plugin_info: Plugin::build_plugin_info_response(),
            config_spec: Plugin::build_config_spec()
        }
    }

    fn build_plugin_info_response() -> PluginInfoResponse {
        PluginInfoResponse {
            r#type: PluginType::Driver as i32,
            plugin_api_versions: vec![String::from(API_VERSION)],
            plugin_version: String::from(PLUGIN_VERSION),
            name: String::from(PLUGIN_NAME)
        }
    }

    fn build_config_spec() -> HashMap<&'static str, Spec> {
        let mut attrs: HashMap<&str, Spec> = HashMap::new();

        attrs.insert(
            "enabled",
            hclext::default_spec(
                Default{
                    primary: Some(Box::from(
                        hclext::new_attr_spec(
                            String::from("enabled"),
                            String::from("bool"),
                            false
                        )
                    )
                    ),
                    default: Some(Box::from(hclext::new_literal_spec(String::from("true"))))
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
            hclext::new_attr_spec(
                String::from("password"),
                String::from("string"),
                true
            )
        );

        attrs.insert(
            "auth",
            hclext::new_object_spec(auth_map)
        );

        attrs
    }
}


