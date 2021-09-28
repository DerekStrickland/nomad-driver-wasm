use std::collections::HashMap;
use std::time::Duration;

use super::health::*;
use super::*;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use tonic::codegen::http::header::HOST;

// EMPTY_DURATION is to be used by fingerprinters that are not periodic.
pub const EMPTY_DURATION: Duration = Duration::new(0, 0);

// TIGHTEN_NETWORK_TIMEOUTS_CONFIG is a config key that can be used during
// tests to tighten the timeouts for fingerprinters that make network calls.
pub const TIGHTEN_NETWORK_TIMEOUTS_CONFIG: &str = "test.tighten_network_timeouts".to_str();

// HOST_FINGERPRINTERS contains the host fingerprints which are available for a
// given platform.
const HOST_FINGERPRINTERS: HashMap<&str, dyn Fingerprinter> = [
    "arch".to_string(): arch::ArchFingerprinter,
    "consul".to_string(): NewConsulFingerprint,
    "cni".to_string(): NewCNIFingerprint,
    "cpu".to_string(): NewCPUFingerprint,
    "host".to_string(): NewHostFingerprint,
    "memory".to_string(): NewMemoryFingerprint,
    "network".to_string(): NewNetworkFingerprint,
    "nomad".to_string(): NewNomadFingerprint,
    "signal".to_string(): NewSignalFingerprint,
    "storage".to_string(): NewStorageFingerprint,
    "vault".to_string(): NewVaultFingerprint,
]
.iter()
.cloned()
.collect();

// ENV_FINGERPRINTERS contains the fingerprints that are environment specific.
// This should run after the host fingerprinters as they may override specific
// node resources with more detailed information.
const ENV_FINGERPRINTERS: HashMap<&str, dyn Fingerprinter> = [
    "env_aws": NewEnvAWSFingerprint,
    "env_gce": NewEnvGCEFingerprint,
    "env_azure": NewEnvAzureFingerprint,
]
.iter()
.cloned()
.collect();

// // builtin_fingerprinters is a vector containing the key names of all registered
// // fingerprints available. The order of this vector should be preserved when
// // fingerprinting.
// pub fn builtin_fingerprinters() -> Vec<String> {
//     let mut fingerprinters = vec![];
//
//     for fingerprinter in hostFingerprinters {
//         fingerprinters.push(fingerprinter);
//     }
//
//     fingerprinters.sort();
//
//     for fingerprinter in ENV_FINGERPRINTERS {
//         fingerprinters.push(fingerprinter);
//     }
//
//     fingerprints
// }

// new is used to instantiate and return a new fingerprint given the name
fn new(name: String) -> Result<dyn Fingerprinter, Err> {
    if HOST_FINGERPRINTERS.contains_key(name.as_str()) {
        host_fingerprinter == HOST_FINGERPRINTERS[name.as_str()];
        Ok(host_fingerprinter::new())
    }

    if ENV_FINGERPRINTERS.contains_key(name.as_str()) {
        env_fingerprinter == ENV_FINGERPRINTERS[name.as_str()];
        Ok(env_fingerprinter::new())
    }

    Err(format!("unknown fingerprint '{}'", name))
}

// Fingerprinter is used for doing "fingerprinting" of the host to automatically
// determine attributes, resources, and metadata about it. Each of these is a
// heuristic, and many of them can be applied on a particular host.
// Fingerprinters should implement both Fingerprinter and either PeriodicFingerprinter
// or StaticFingerprinter so tha the periodic fn is satisfied.
pub trait Fingerprinter {
    fn new() -> dyn Fingerprinter;
    // Fingerprint is used to update properties of the Node,
    // and returns a diff of updated node attributes and a potential error.
    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, Err>;
}

// PeriodicFingerprinter can be implemented on a struct that has a Fingerprint method
// to mark it as periodic.
pub trait PeriodicFingerprinter: Fingerprinter {
    // Periodic is a mechanism for the fingerprinter to indicate that it should
    // be run periodically. The return value is a boolean indicating if it
    // should be periodic, and if true, a duration.
    fn periodic(&self) -> (bool, Duration);
}

// StaticFingerprinter can be implemented on a struct that has a Fingerprint method
// to mark it as non-periodic.
pub trait StaticFingerprinter: Fingerprinter {
    fn periodic(&self) -> (bool, Duration) {
        (false, Duration::new(0, 0))
    }
}

// ReloadableFingerprint can be implemented if the fingerprinter needs to be run
// during client reload. If implemented, the client will call Reload during client
// reload then immediately Fingerprint
pub trait ReloadableFingerprinter: Fingerprinter {
    fn reload(&self);
}
