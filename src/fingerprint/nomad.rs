use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// NomadFingerprinter is used to fingerprint the host Nomad configuration.
pub struct NomadFingerprinter {}

impl NomadFingerprinter {
    pub fn new() -> Self {
        NomadFingerprinter {}
    }
}

impl Fingerprinter for NomadFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Nomad fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for NomadFingerprinter {}
