#![allow(dead_code)]
use env_logger;
use log;

use proto::hashicorp::nomad::plugins::drivers::proto::driver_client::DriverClient;
use proto::hashicorp::nomad::plugins::drivers::proto::FingerprintRequest;
use proto::hashicorp::nomad::plugins::drivers::proto::{CapabilitiesRequest, DriverCapabilities};

mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let channel = tonic::transport::Channel::from_static("http://[::1]:5000")
        .connect()
        .await?;

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
        log::info!("health: {}", fingerprint_response.health);
    }

    Ok(())
}
