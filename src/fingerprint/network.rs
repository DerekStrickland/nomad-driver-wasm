use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// NetworkFingerprinter is used to fingerprint the host Network configuration.
pub struct NetworkFingerprinter {}

impl NetworkFingerprinter {
    pub fn new() -> Self {
        NetworkFingerprinter {}
    }
}

impl Fingerprinter for NetworkFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Network fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for NetworkFingerprinter {}
