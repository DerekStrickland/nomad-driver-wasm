use std::collections::HashMap;

// Alias nomad modules
use crate::driver::Spec;
use crate::hclext;
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::drivers::proto as drivers;

use drivers::TaskConfigSchemaResponse;

// Handles task_config_schema requests.
pub fn task_config_schema() -> TaskConfigSchemaResponse {
    let mut attrs: HashMap<String, Spec> = HashMap::new();

    // Name of the task as registered with Nomad.
    attrs.insert(
        String::from("name"),
        hclext::new_attr_spec(String::from("name"), String::from("string"), true),
    );

    // Image url of wasm module to run.
    attrs.insert(
        String::from("image"),
        hclext::new_attr_spec(String::from("image"), String::from("string"), true),
    );

    let task_spec = hclext::new_object_spec(attrs);

    TaskConfigSchemaResponse {
        spec: Option::from(task_spec),
    }
}
