use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto")
        .compile_well_known_types(true)
        .include_file("mod.rs")
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .compile(
            &[
                "nomad/plugins/base/proto/base.proto",
                "nomad/plugins/drivers/proto/driver.proto",
                "nomad/plugins/shared/hclspec/hcl_spec.proto",
                "nomad/plugins/shared/structs/proto/attribute.proto",
            ],
            &["nomad"],
        )
        .unwrap();

    Ok(())
}