use super::fingerprinter::FingerprintError;
use super::fingerprinter::Fingerprinter;
use crate::fingerprint::fingerprinter::PeriodicFingerprinter;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use std::time::Duration;

// ConsulFingerprinter is used to fingerprint the host Consul configuration.
pub struct ConsulFingerprinter {}

impl ConsulFingerprinter {
    pub fn new() -> Self {
        ConsulFingerprinter {}
    }
}

impl Fingerprinter for ConsulFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Consul fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl PeriodicFingerprinter for ConsulFingerprinter {
    fn periodic(&self) -> (bool, Duration) {
        todo!()
    }
}
