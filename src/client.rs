#![allow(dead_code)]
use env_logger;
use log;

use proto::drivers::{CapabilitiesRequest};
use proto::drivers::driver_client::{DriverClient};

mod proto;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let channel = tonic::transport::Channel::from_static("http://[::1]:5000")
        .connect()
        .await?;

    let mut client = DriverClient::new(channel);

    let request = tonic::Request::new(
        CapabilitiesRequest {},
    );

    let response = client.capabilities(request).await?.into_inner();

    log::info!("exec: {}", response.capabilities.unwrap_or_else(|| {
        log::info!("capabilities unavailable: returning default");
        plugin::Plugin::new().capabilities
    }).exec);
    Ok(())
}