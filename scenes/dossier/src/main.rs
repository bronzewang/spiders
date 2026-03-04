use std::path::PathBuf;

use anyhow::Result;
use console_subscriber::ConsoleLayer;
use futures::StreamExt;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use tarpc::{
    context::Context,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::UnixListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};
// use futures::prelude::*;

#[tarpc::service]
trait DossierService {
    async fn handshake();
}

#[derive(Clone)]
struct Service;

impl DossierService for Service {
    async fn handshake(self, _context: Context) -> () {}
}

struct OtelGuard {
    logger_provider: SdkLoggerProvider,
    // metrics_provider: SdkMeterProvider,
    // tracer_provider: SdkTracerProvider,
}
impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(e) = self.logger_provider.shutdown() {
            eprintln!("logger_provider shutdown fail error {e}");
        }
    }
}

fn init_tracing_subscriber() -> OtelGuard {
    let logger_provider = init_logger_provider();

    let otel_filter = EnvFilter::new("debug")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider).with_filter(otel_filter);

    let term_filter = EnvFilter::new("debug");
    let term_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_filter(term_filter);

    let console_layzer = ConsoleLayer::builder()
        .retention(ConsoleLayer::DEFAULT_RETENTION)
        .server_addr(([127, 0, 0, 1], 6669))
        .spawn();

    tracing_subscriber::registry()
        .with(console_layzer)
        .with(otel_layer)
        .with(term_layer)
        .init();

    OtelGuard { logger_provider }
}

const TONIC_ENDPOINT: &str = "http://192.168.3.47:4317";

fn resource() -> Resource {
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .build()
}

fn init_logger_provider() -> SdkLoggerProvider {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(TONIC_ENDPOINT)
        .build()
        .expect("init log exporter fail");
    SdkLoggerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

#[tokio::main]
async fn main() -> Result<()> {
    let _otel_guard = init_tracing_subscriber();

    let listener_path = PathBuf::from("/tmp").join("spiders");
    tokio::fs::create_dir_all(&listener_path).await?;
    let listener_addr = listener_path
        .join(env!("CARGO_PKG_NAME"))
        .with_extension("sock");
    tokio::fs::remove_file(&listener_addr).await?;

    let listener = UnixListener::bind(listener_addr).unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    loop {
        let (conn, _addr) = listener.accept().await?;
        let framed = codec_builder.new_framed(conn);
        let transport = tarpc::serde_transport::new(framed, Bincode::default());

        let channel_fut = BaseChannel::with_defaults(transport)
            .execute(Service.serve())
            .for_each(spawn);

        tokio::spawn(channel_fut);
    }

    async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
        tokio::spawn(fut);
    }
}

// use clap::Parser;
// use dora_node_api::{DoraNode, Event};
// // use opentelemetry_appender_log::OpenTelemetryLogBridge;
// // use opentelemetry_sdk::{logs::SdkLoggerProvider, metrics::SdkMeterProvider, Resource};
// use prometheus::Registry;
// // use opentelemetry::{
// //     metrics::{Counter, Histogram, MeterProvider as _, Unit},
// //     KeyValue,
// // };
// use log::{Level, error};
// use std::error::Error;
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tonic::transport::Server;

// // use spiders_dossier::{app::*};

// pub mod greeter {
//     include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
// }

// pub use greeter::*;

// use crate::greeter_server::Greeter;
// use crate::greeter_server::GreeterServer;

// #[derive(Debug, Parser)]
// struct Cli {
//     #[arg(short, long)]
//     addr: Option<SocketAddr>,
// }

// #[derive(Debug)]
// struct AppState {
//     // toolkit: Vec<Toolkit>,
//     // snooper: Vec<Snooper>,
//     registry: Registry,
//     // tonic_counter: Counter<u64>,
//     // tonic_body_gauge: Histogram<u64>,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let cli = Cli::parse();

//     // toolkit_init().await?;

//     // let exporter = opentelemetry_stdout::LogExporter::default();
//     // let logger_provider = SdkLoggerProvider::builder()
//     //     .with_resource(
//     //         Resource::builder()
//     //             .with_service_name("log-appender-tracing-example")
//     //             .build(),
//     //     )
//     //     .with_simple_exporter(exporter)
//     //     .build();

//     // // Setup Log Appender for the log crate.
//     // let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
//     // log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
//     // log::set_max_level(Level::Error.to_level_filter());

//     let registry = Registry::new();
//     // let exporter = opentelemetry_prometheus::exporter()
//     //     .with_registry(registry.clone())
//     //     .build()?;
//     // let provider = SdkMeterProvider::builder().with_reader(exporter).build();

//     // let meter = provider.meter("spiders-dossier");
//     let state = Arc::new(AppState {
//         // toolkit: Vec::new(),
//         // snooper: Vec::new(),
//         registry: registry,
//         // tonic_counter: meter
//         //     .u64_counter("tonic_requests_total")
//         //     .with_description("Total number of TONIC requests made.")
//         //     .init(),
//         // tonic_body_gauge: meter
//         //     .u64_histogram("example.tonic_response_size")
//         //     .with_unit(Unit::new("By"))
//         //     .with_description("The metrics TONIC response sizes in bytes.")
//         //     .init(),
//     });

//     let addr = cli.addr.unwrap_or_else(|| "[::1]:29000".parse().unwrap());
//     // addr.set_port(addr.port()+innate.id);

//     tokio::select! {
//         dora_result = dora_server(state.clone()) => {
//             // println!("dora task completed with result: {}", dora_result);
//         },
//         tonic_result = tonic_server(state, addr) => {
//             // println!("tonic task completed with result: {}", tonic_result);
//         },
//     }

//     println!("spider dossier Doone!!!");

//     Ok(())
// }

// async fn dora_server(state: Arc<AppState>) -> Result<(), Box<dyn Error>> {
//     let (_node, mut events) = DoraNode::init_from_env()?;
//     // let (_node, mut events) =
//     //     DoraNode::init_from_node_id(NodeId::from("spiders-dossier-dyn".to_string()))?;

//     while let Some(event) = events.recv_async().await {
//         match event {
//             Event::Input {
//                 id,
//                 metadata: _,
//                 data,
//             } => match id.as_str() {
//                 // "message" => {
//                 //     let received_string: &str =
//                 //         TryFrom::try_from(&data).context("expected string message")?;
//                 //     println!("sink received message: {received_string}");
//                 //     if !received_string.starts_with("operator received random value ") {
//                 //         bail!("unexpected message format (should start with 'operator received random value')")
//                 //     }
//                 //     if !received_string.ends_with(" ticks") {
//                 //         bail!("unexpected message format (should end with 'ticks')")
//                 //     }
//                 // }
//                 // other => eprintln!("Ignoring unexpected input `{other}`"),
//                 other => println!("Input `{other}`"),
//             },
//             Event::Stop(_) => {
//                 println!("Received stop");
//             }
//             Event::InputClosed { id } => {
//                 println!("Input `{id}` was closed");
//             }
//             other => eprintln!("Received unexpected input: {other:?}"),
//         }
//     }

//     Ok(())
// }

// #[derive(Default)]
// struct MyGreeter {}

// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     /// 登签
//     async fn sign(
//         &self,
//         request: tonic::Request<SignPing>,
//     ) -> std::result::Result<tonic::Response<SignPong>, tonic::Status> {
//         Ok(tonic::Response::new(SignPong { timestamp: 0 }))
//     }

//     /// 链路维持
//     async fn keep(
//         &self,
//         request: tonic::Request<KeepPing>,
//     ) -> std::result::Result<tonic::Response<KeepPong>, tonic::Status> {
//         Ok(tonic::Response::new(KeepPong { timestamp: 0 }))
//     }

//     /// Toolkit列表
//     async fn toolkit_list(
//         &self,
//         request: tonic::Request<ToolkitListPing>,
//     ) -> std::result::Result<tonic::Response<ToolkitListPong>, tonic::Status> {
//         Ok(tonic::Response::new(ToolkitListPong {
//             toolkits: Vec::new(),
//         }))
//     }

//     /// Snooper列表
//     async fn snooper_list(
//         &self,
//         request: tonic::Request<SnooperListPing>,
//     ) -> std::result::Result<tonic::Response<SnooperListPong>, tonic::Status> {
//         Ok(tonic::Response::new(SnooperListPong {
//             snoopers: Vec::new(),
//         }))
//     }
// }

// async fn tonic_server(state: Arc<AppState>, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
//     error!("listening on {}", addr);
//     let greeter = MyGreeter::default();
//     Server::builder()
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;
//     Ok(())
// }
