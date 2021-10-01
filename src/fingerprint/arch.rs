use super::fingerprinter::FingerprintError;
use super::fingerprinter::{Fingerprinter, StaticFingerprinter};
use crate::proto::hashicorp::nomad::plugins::drivers::proto::{
    FingerprintRequest, FingerprintResponse,
};
use crate::proto::hashicorp::nomad::plugins::shared::structs::attribute::Value;
use crate::proto::hashicorp::nomad::plugins::shared::structs::Attribute;

// ArchFingerprinter is used to fingerprint the host CPU architecture
pub struct ArchFingerprinter {}

impl ArchFingerprinter {
    pub fn new() -> Self {
        ArchFingerprinter {}
    }
}

impl Fingerprinter for ArchFingerprinter {
    fn fingerprint(
        &self,
        _request: FingerprintRequest,
        response: FingerprintResponse,
    ) -> Result<FingerprintResponse, FingerprintError> {
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
