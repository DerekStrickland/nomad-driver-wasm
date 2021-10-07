#![allow(dead_code)]
// use env_logger;
// use env_logger::{Builder, Target};
// use log;
// use log::LevelFilter;
// use log4rs::append::console::ConsoleAppender;
// use log4rs::append::file::FileAppender;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;
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
    // log::info!("Health check received");
    reporter.set_serving::<DriverServer<WasmDriver>>().await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // go-plugin requires this to be written to satisfy the handshake protocol.
    // https://github.com/hashicorp/go-plugin/blob/master/docs/guide-plugin-write-non-go.md#4-output-handshake-information
    println!("1|2|tcp|127.0.0.1:5000|grpc");

    // let stdout = ConsoleAppender::builder().build();
    //
    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{l} - {m}{n}\n")))
    //     .build("/var/log/nomad-driver-wasm.log")?;
    //
    // let config = Config::builder()
    //     .appender(Appender::builder().build("stdout", Box::new(stdout)))
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(
    //         Root::builder()
    //             .appender("stdout")
    //             //.appender("logfile")
    //             .build(LevelFilter::Debug),
    //     )?;
    //
    // log4rs::init_config(config)?;

    // env_logger::init();

    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.filter_level(LevelFilter::Debug);
    //
    // builder.init();

    // log::info!("Starting nomad-driver-wasm server");

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DriverServer<WasmDriver>>()
        .await;

    tokio::spawn(driver_service_status(health_reporter.clone()));

    let addr = "0.0.0.0:5000".parse().unwrap();
    let driver_server = WasmDriver::default();
    let plugin_server = WasmDriver::default();

    // log::info!("nomad-driver-wasm server listening on {}", addr);

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
