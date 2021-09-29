use super::fingerprinter::FingerprintError;
use super::fingerprinter::Fingerprinter;
use crate::fingerprint::fingerprinter::PeriodicFingerprinter;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use std::time::Duration;

// CgroupFingerprinter tries to find a valid cgroup mount point.
pub struct CgroupFingerprinter {}

impl Fingerprinter for CgroupFingerprinter {
    fn new() -> Self {
        CgroupFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Cgroup fingerprint not supported yet for wasm workloads");

        Ok(result)
    }
}

impl PeriodicFingerprinter for CgroupFingerprinter {
    fn periodic(&self) -> (bool, Duration) {
        todo!()
    }
}
