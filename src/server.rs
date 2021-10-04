#![allow(dead_code)]
use env_logger;
use log;
// use std::io::{self, Write};
use std::time::Duration;
use tonic::transport::Server;
use tonic_health::server::HealthReporter;

use driver::WasmDriver;
use proto::hashicorp::nomad::plugins::base::proto::base_plugin_server::BasePluginServer;
use proto::hashicorp::nomad::plugins::drivers::proto::driver_server::DriverServer;

mod driver;
mod fingerprint;
mod hclext;
mod proto;

async fn driver_service_status(mut reporter: HealthReporter) {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        reporter.set_serving::<DriverServer<WasmDriver>>().await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // go-plugin requires this to be written to satisfy the handshake protocol.
    // https://github.com/hashicorp/go-plugin/blob/master/docs/guide-plugin-write-non-go.md#4-output-handshake-information
    println!("1|2|tcp|127.0.0.1:5000|grpc");

    env_logger::init();

    log::info!("Starting nomad-driver-wasm server");

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DriverServer<WasmDriver>>()
        .await;

    tokio::spawn(driver_service_status(health_reporter.clone()));

    let addr = "0.0.0.0:5000".parse().unwrap();
    let driver_server = WasmDriver::default();
    let plugin_server = WasmDriver::default();

    log::info!("Server listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(DriverServer::new(driver_server))
        .add_service(BasePluginServer::new(plugin_server))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::main;
    use tokio_test::assert_ok;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_main() {
        assert_ok!(main())
    }
}
