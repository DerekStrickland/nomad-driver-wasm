use prost_types::Duration;
use serde::{Deserialize, Serialize};

// Serde calls this the definition of the remote type. It is just a copy of the
// remote data structure. The `remote` attribute gives the path to the actual
// type we intend to derive code for.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Duration")]
pub struct DurationDef {
    secs: i64,
    nanos: i32,
}
