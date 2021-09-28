use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use crate::proto::hashicorp::nomad::plugins::shared::structs::attribute::Value;
use crate::proto::hashicorp::nomad::plugins::shared::structs::Attribute;
use std::time::Duration;

// CniFingerprinter is used to fingerprint the host CNI configuration.
pub struct CniFingerprinter {}

impl Fingerprinter for CniFingerprinter {
    // new is used to create an OS fingerprint
    fn new() -> CniFingerprinter {
        CniFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, Err> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("CNI fingerprint not valid for wasmtime workloads");

        result.Ok(result)
    }
}

impl StaticFingerprinter for CniFingerprinter {}
