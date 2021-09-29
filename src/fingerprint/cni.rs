use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

use super::fingerprinter::FingerprintError;

// CniFingerprinter is used to fingerprint the host CNI configuration.
pub struct CniFingerprinter {}

impl Fingerprinter for CniFingerprinter {
    fn new() -> Self {
        CniFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description = String::from("CNI fingerprint not valid for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for CniFingerprinter {}
