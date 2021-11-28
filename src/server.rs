#![allow(dead_code)]
use tonic::transport::Server;
use tonic_health::server::HealthReporter;

// Alias nomad modules
use crate::proto::hashicorp::nomad::plugins as nomad;
use nomad::base::proto as base;
use nomad::drivers::proto as drivers;

use base::base_plugin_server::BasePluginServer;
use driver::WasmDriver;
use drivers::driver_server::DriverServer;

mod driver;
mod fingerprint;
mod hclext;
mod proto;
mod task;

async fn driver_service_status(mut reporter: HealthReporter) {
    println!("Health check received");
    reporter.set_serving::<DriverServer<WasmDriver>>().await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // go-plugin requires this to be written to satisfy the handshake protocol.
    // https://github.com/hashicorp/go-plugin/blob/master/docs/guide-plugin-write-non-go.md#4-output-handshake-information
    println!("1|2|tcp|127.0.0.1:5001|grpc");

    println!("Starting nomad-driver-wasm server");

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DriverServer<WasmDriver>>()
        .await;

    tokio::spawn(driver_service_status(health_reporter.clone()));

    let addr = "0.0.0.0:5001".parse().unwrap();
    let driver_server = WasmDriver::default();
    let plugin_server = WasmDriver::default();

    println!("nomad-driver-wasm server listening on {}", addr);

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
