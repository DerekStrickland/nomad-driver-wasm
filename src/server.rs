#![allow(dead_code)]
use env_logger;
use log;
use tonic::transport::Server;

use driver::WasmtimeDriver;
use proto::hashicorp::nomad::plugins::drivers::proto::driver_server::{DriverServer};

mod driver;
mod proto;
mod hclext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    log::info!("Starting nomad-driver-wasmtime server");

    let addr = "[::1]:5000".parse().unwrap();
    let driver = WasmtimeDriver::default();

    log::info!("Server listening on {}", addr);

    Server::builder()
        .add_service(DriverServer::new(driver))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{main};
    use tokio_test::{assert_ok};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_main() {
        assert_ok!(main())
    }
}
