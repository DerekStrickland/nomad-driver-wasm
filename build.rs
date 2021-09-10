use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto")
        .extern_path(".hashicorp.nomad.plugins.shared.hclspec", "crate::proto::hclspec")
        .extern_path(".hashicorp.nomad.plugins.shared.structs", "crate::proto::structs")
        .compile(
            &[
                "nomad/plugins/drivers/proto/driver.proto",
            ],
            &["nomad"],
        )
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto")
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