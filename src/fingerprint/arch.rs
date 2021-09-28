use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use crate::proto::hashicorp::nomad::plugins::shared::structs::attribute::Value;
use crate::proto::hashicorp::nomad::plugins::shared::structs::Attribute;
use std::time::Duration;

// ArchFingerprinter is used to fingerprint the host CPU architecture
pub struct ArchFingerprinter {}

impl Fingerprinter for ArchFingerprinter {
    // new is used to create an OS fingerprint
    fn new() -> ArchFingerprinter {
        ArchFingerprinter {}
    }

    fn fingerprint(
        &self,
        request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, Err> {
        let mut result = response.clone();

        result.attributes.insert(
            String::from("cpu.arch"),
            Attribute {
                unit: "".to_string(),
                value: Option::from(Value::StringVal(String::from(std::env::consts::ARCH))),
            },
        );

        Ok(result)
    }
}

impl StaticFingerprinter for ArchFingerprinter {}
