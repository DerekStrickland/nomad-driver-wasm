use std::error::Error;
use std::time::Duration;

use super::*;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use std::fmt::{Debug, Display, Formatter};

// TIGHTEN_NETWORK_TIMEOUTS_CONFIG is a config key that can be used during
// tests to tighten the timeouts for fingerprinters that make network calls.
const TIGHTEN_NETWORK_TIMEOUTS_CONFIG: &str = "test.tighten_network_timeouts";

// TODO: Research purpose of builtin_fingerprinters in Nomad.
// builtin_fingerprinters is a vector containing the key names of all registered
// fingerprints available. The order of this vector should be preserved when
// fingerprinting.
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
fn new(name: String) -> Result<Box<dyn Fingerprinter>, FingerprintError> {
    let name = name.as_str();

    if name.is_empty() {
        return Err(FingerprintError::new(format!(
            "invalid fingerprinter specified: {}",
            name
        )));
    }

    match name {
        "arch" => Ok(Box::new(arch::ArchFingerprinter::new())),
        "cni" => Ok(Box::new(cni::CniFingerprinter::new())),
        "consul" => Ok(Box::new(consul::ConsulFingerprinter::new())),
        "cpu" => Ok(Box::new(cpu::CpuFingerprinter::new())),
        "host" => Ok(Box::new(host::HostFingerprinter::new())),
        "memory" => Ok(Box::new(memory::MemoryFingerprinter::new())),
        "network" => Ok(Box::new(network::NetworkFingerprinter::new())),
        "nomad" => Ok(Box::new(nomad::NomadFingerprinter::new())),
        "signal" => Ok(Box::new(signal::SignalFingerprinter::new())),
        "storage" => Ok(Box::new(storage::StorageFingerprinter::new())),
        "vault" => Ok(Box::new(vault::VaultFingerprinter::new())),
        // "env_aws": aws::AwsFingerprinter::new(),
        // "env_gce": gce::GceFingerprinter::new,
        // "env_azure": azure::AzureFingerprinter::new(),
        _ => Err(FingerprintError::new(format!(
            "no match for specified fingerprinter: {}",
            name
        ))),
    }
}

// Fingerprinter is used for doing "fingerprinting" of the host to automatically
// determine attributes, resources, and metadata about it. Each of these is a
// heuristic, and many of them can be applied on a particular host.
// Fingerprinters should implement both Fingerprinter and either PeriodicFingerprinter
// or StaticFingerprinter so tha the periodic fn is satisfied.
pub trait Fingerprinter {
    // Fingerprint is used to update properties of the Node,
    // and returns a diff of updated node attributes and a potential error.
    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError>;
}

// StaticFingerprinter can be implemented on a struct that has a Fingerprint method
// to mark it as non-periodic and satisfy the required interface with this default
// implementation.
pub trait StaticFingerprinter: Fingerprinter {
    fn periodic(&self) -> (bool, Duration) {
        (false, Duration::new(0, 0))
    }
}

// PeriodicFingerprinter can be implemented on a struct that has a Fingerprint method
// to mark it as periodic and requiring a custom implementation to satisfy the
// required interface.
pub trait PeriodicFingerprinter: Fingerprinter {
    fn periodic(&self) -> (bool, Duration);
}

// ReloadableFingerprint can be implemented if the fingerprinter needs to be run
// during client reload. If implemented, the client will call Reload during client
// reload then immediately Fingerprint
pub trait ReloadableFingerprinter: Fingerprinter {
    fn reload(&self);
}

pub struct FingerprintError {
    message: String,
}

impl FingerprintError {
    pub fn new(message: String) -> FingerprintError {
        FingerprintError { message }
    }
}

impl Debug for FingerprintError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FingerprintError: {}", &self.message)
    }
}

impl Display for FingerprintError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FingerprintError: {}", &self.message)
    }
}

impl Error for FingerprintError {}
