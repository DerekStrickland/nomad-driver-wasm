use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto")
        .compile_well_known_types(true)
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .extern_path(".hashicorp.nomad.plugins.shared.hclspec", "crate::proto::hclspec")
        .extern_path(".hashicorp.nomad.plugins.shared.structs", "crate::proto::structs")
        .extern_path(".google.protobuf", "crate::proto::google_protobuf")
        .compile(
            &[
                "nomad/plugins/base/proto/base.proto",
                "nomad/plugins/drivers/proto/driver.proto",
            ],
            &["nomad"],
        )
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto")
        .compile_well_known_types(true)
        .type_attribute(".", "#[derive(::serde::Deserialize)]")
        .compile(
            &[
                "nomad/plugins/shared/hclspec/hcl_spec.proto",
                "nomad/plugins/shared/structs/proto/attribute.proto",
            ],
            &["nomad"],
        )
        .unwrap();

    Ok(())
}