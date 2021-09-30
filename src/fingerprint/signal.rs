use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// SignalFingerprinter is used to fingerprint the available signals.
pub struct SignalFingerprinter {}

impl SignalFingerprinter {
    pub fn new() -> Self {
        SignalFingerprinter {}
    }
}

impl Fingerprinter for SignalFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Signal fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for SignalFingerprinter {}
