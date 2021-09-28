use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::fingerprint_response::HealthState;
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use crate::proto::hashicorp::nomad::plugins::shared::structs::attribute::Value;
use crate::proto::hashicorp::nomad::plugins::shared::structs::Attribute;
use std::time::Duration;

// ConsulFingerprinter is used to fingerprint the host CNI configuration.
pub struct ConsulFingerprinter {}

impl Fingerprinter for ConsulFingerprinter {
    // new is used to create an OS fingerprint
    fn new() -> ConsulFingerprinter {
        ConsulFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, Err> {
        let mut result = response.clone();

        result.health = HealthState::Undetected as i32;
        result.health_description =
            String::from("Consul fingerprint not supported yet for wasmtime workloads");

        result.Ok(result)
    }
}

impl StaticFingerprinter for ConsulFingerprinter {}
