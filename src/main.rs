#![allow(dead_code)]
mod driver;
mod plugin;
mod proto;

use env_logger;
use log;
use tonic::transport::Server;

use crate::proto::drivers::driver_server::{DriverServer};
use crate::driver::WasmtimeDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

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
