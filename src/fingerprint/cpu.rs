extern crate sys_info;
use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};

// CpuFingerprinter is used to fingerprint the host CPU resources.
pub struct CpuFingerprinter {}

impl Fingerprinter for CpuFingerprinter {
    fn new() -> Self {
        CpuFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("CPU fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl StaticFingerprinter for CpuFingerprinter {}
