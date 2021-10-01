use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// StorageFingerprinter is used to fingerprint the host storage resources.
pub struct StorageFingerprinter {}

impl StorageFingerprinter {
    pub fn new() -> Self {
        StorageFingerprinter {}
    }
}

impl Fingerprinter for StorageFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Storage fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for StorageFingerprinter {}
