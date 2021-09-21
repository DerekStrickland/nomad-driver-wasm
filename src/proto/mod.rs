#[path = "hashicorp.nomad.plugins.base.proto.rs"]
pub mod base;

#[path = "hashicorp.nomad.plugins.drivers.proto.rs"]
pub mod drivers;

pub mod hclext;

#[path = "hashicorp.nomad.plugins.shared.hclspec.rs"]
pub mod hclspec;

#[path = "hashicorp.nomad.plugins.shared.structs.rs"]
pub mod structs;

mod driversext;

#[path = "google.protobuf.rs"]
mod google_protobuf;
