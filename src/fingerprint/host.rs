use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// HostFingerprinter is used to fingerprint the host.
pub struct HostFingerprinter {}

impl HostFingerprinter {
    pub fn new() -> Self {
        HostFingerprinter {}
    }
}

impl Fingerprinter for HostFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Host fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for HostFingerprinter {}
