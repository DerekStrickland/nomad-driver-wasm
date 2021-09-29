use super::fingerprinter::FingerprintError;
use super::fingerprinter::Fingerprinter;
use crate::fingerprint::fingerprinter::PeriodicFingerprinter;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use std::time::Duration;

// VaultFingerprinter is used to fingerprint the host Vault configuration.
pub struct VaultFingerprinter {}

impl Fingerprinter for VaultFingerprinter {
    fn new() -> Self {
        VaultFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Vault fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl PeriodicFingerprinter for VaultFingerprinter {
    fn periodic(&self) -> (bool, Duration) {
        todo!()
    }
}
