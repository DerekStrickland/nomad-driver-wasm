use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// MemoryFingerprinter is used to fingerprint the host Memory resources.
pub struct MemoryFingerprinter {}

impl Fingerprinter for MemoryFingerprinter {
    fn new() -> Self {
        MemoryFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Memory fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for MemoryFingerprinter {}
