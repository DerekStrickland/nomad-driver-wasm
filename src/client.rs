#![allow(dead_code)]
use env_logger;
// use env_logger::{Builder, Target};
use log;
// use log::LevelFilter;

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::drivers::proto as drivers;

use drivers::driver_client::DriverClient;
use drivers::FingerprintRequest;
use drivers::{CapabilitiesRequest, DriverCapabilities};

mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.filter_level(LevelFilter::Debug);

    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:5000")
        .connect()
        .await?;

    log::info!("channel established");

    let mut client = DriverClient::new(channel);

    let capabilities_request = tonic::Request::new(CapabilitiesRequest {});

    let capabilities_response = client
        .capabilities(capabilities_request)
        .await?
        .into_inner();

    log::info!(
        "exec: {}",
        capabilities_response
            .capabilities
            .unwrap_or_else(|| {
                log::info!("capabilities unavailable: returning default");
                DriverCapabilities {
                    exec: false,
                    fs_isolation: 0,
                    mount_configs: 0,
                    must_create_network: false,
                    network_isolation_modes: vec![],
                    remote_tasks: false,
                    send_signals: false,
                }
            })
            .exec
    );

    let fingerprint_request = tonic::Request::new(FingerprintRequest {});
    let mut fingerprint_response_stream =
        client.fingerprint(fingerprint_request).await?.into_inner();

    while let Some(fingerprint_response) = fingerprint_response_stream.message().await? {
        log::info!(
            "health_description: {}",
            fingerprint_response.health_description
        );
    }

    Ok(())
}
