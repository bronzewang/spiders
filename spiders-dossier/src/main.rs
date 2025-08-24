use clap::Parser;
use dora_node_api::{DoraNode, Event};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::{logs::SdkLoggerProvider, metrics::SdkMeterProvider, Resource};
use prometheus::{Registry};
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider as _, Unit},
    KeyValue,
};
use tonic::transport::Server;
use std::sync::Arc;
use std::error::Error;
use std::net::SocketAddr;
use log::{error, Level};

use spiders_dossier::{app::*};

pub mod greeter {
	include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}

pub use greeter::*;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    addr: Option<SocketAddr>,
}

#[derive(Debug)]
struct AppState {
	// toolkit: Vec<Toolkit>,
	// snooper: Vec<Snooper>,

    registry: Registry,
    tonic_counter: Counter<u64>,
    tonic_body_gauge: Histogram<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();

	// toolkit_init().await?;

    let exporter = opentelemetry_stdout::LogExporter::default();
    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("log-appender-tracing-example")
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Error.to_level_filter());

    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()?;
    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    let meter = provider.meter("spiders-dossier");
    let state = Arc::new(AppState {
		// toolkit: Vec::new(),
		// snooper: Vec::new(),
        registry: registry,
        tonic_counter: meter
            .u64_counter("tonic_requests_total")
            .with_description("Total number of TONIC requests made.")
            .init(),
        tonic_body_gauge: meter
            .u64_histogram("example.tonic_response_size")
            .with_unit(Unit::new("By"))
            .with_description("The metrics TONIC response sizes in bytes.")
            .init(),
    });

    let addr = cli.addr.unwrap_or_else(|| "[::1]:29000".parse().unwrap());
	// addr.set_port(addr.port()+innate.id);

    tokio::select! {
        dora_result = dora_server(state) => {
            println!("dora task completed with result: {}", dora_result);
        },
        tonic_result = tonic_server(state, addr) => {
            println!("tonic task completed with result: {}", tonic_result);
        },
    }

    println!("spider dossier Doone!!!");

    Ok(())
}

async fn dora_server(state: Arc<AppState>) -> Result<(), Error> {
    let (_node, mut events) = DoraNode::init_from_env()?;
    // let (_node, mut events) =
    //     DoraNode::init_from_node_id(NodeId::from("spiders-dossier-dyn".to_string()))?;

    while let Some(event) = events.recv_async().await {
        match event {
            Event::Input {
                id,
                metadata: _,
                data,
            } => match id.as_str() {
                // "message" => {
                //     let received_string: &str =
                //         TryFrom::try_from(&data).context("expected string message")?;
                //     println!("sink received message: {received_string}");
                //     if !received_string.starts_with("operator received random value ") {
                //         bail!("unexpected message format (should start with 'operator received random value')")
                //     }
                //     if !received_string.ends_with(" ticks") {
                //         bail!("unexpected message format (should end with 'ticks')")
                //     }
                // }
                // other => eprintln!("Ignoring unexpected input `{other}`"),
                other => println!("Input `{other}`"),
            },
            Event::Stop(_) => {
                println!("Received stop");
            }
            Event::InputClosed { id } => {
                println!("Input `{id}` was closed");
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }
    Ok(());
}

async fn tonic_server(state: Arc<AppState>, addr: SocketAddr) -> Result<(), Error> {
    error!("listening on {}", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(());
}
