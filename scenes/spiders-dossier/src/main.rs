use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
use serde::Deserialize;
use axum::{response::Html, extract::State, routing::get, Router};
use log::{error, Level};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::{logs::{Config, LoggerProvider}, metrics::SdkMeterProvider};
use opentelemetry_sdk::Resource;
use prometheus::{Registry};
use std::sync::Arc;
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider as _, Unit},
    KeyValue,
};

// use spiders_scheme::SPIDERS_WEB_PORT_BASE;
const SPIDERS_WEB_PORT_BASE: u16 = 62999;

// pub mod greeter {
// 	include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
// }

// pub use greeter::*;

// 上电初始化一次的参数 'static
#[derive(Deserialize, Debug)]
pub struct Innate {
	// 公共信息存放的位置，如数据库文件assets.db
    pub fibase_shrine: PathBuf,
    pub sibase_shrine: PathBuf,

	// 参数存放的位置
    pub fibase_valver: PathBuf,
    pub sibase_valver: PathBuf,

    pub fibase_snaper: PathBuf,
    pub sibase_snaper: PathBuf,

    pub sibase_larder: PathBuf,
    pub fibase_larder: PathBuf,
    pub sxmass_larder: PathBuf,
    pub fxmass_larder: PathBuf,
    pub sxplug_larder: PathBuf,
    pub fxplug_larder: PathBuf,

    pub id: u16,
    pub name: String,
}

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short = 'f', long = "fibase_innate")]
    fibase_innate: Option<PathBuf>,
    #[arg(short = 's', long = "sibase_innate")]
    sibase_innate: Option<PathBuf>,
}

#[derive(Debug)]
struct AppState {
    registry: Registry,
    http_counter: Counter<u64>,
    http_body_gauge: Histogram<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    let innate_path = cli.fibase_innate.unwrap_or(cli.sibase_innate.unwrap_or(Path::new("./scenes/spiders-dossier/utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

    // udev_parse().await?;
    serial_parse().await?;

    let exporter = opentelemetry_stdout::LogExporterBuilder::default().build();
    let logger_provider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "spiders-dossier",
            )]))
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
        registry,
        http_counter: meter
            .u64_counter("http_requests_total")
            .with_description("Total number of HTTP requests made.")
            .init(),
        http_body_gauge: meter
            .u64_histogram("example.http_response_size")
            .with_unit(Unit::new("By"))
            .with_description("The metrics HTTP response sizes in bytes.")
            .init(),
    });

    // build our application with a route
    let app = Router::new().route("/", get(handler)).with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",
                                                         SPIDERS_WEB_PORT_BASE+innate.id))
        .await
        .unwrap();
    // run it
    error!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler(State(state): State<Arc<AppState>>) -> Html<&'static str> {
    error!("{:?}", state);
    error!("{:?} {:?} {:?}", state.registry, state.http_counter, state.http_body_gauge);

    error!("respond");
    Html("<h1>Hello, World! I am Spiders.</h1>")
}

// use futures_util::future::ready;
// use futures_util::stream::StreamExt;
// use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

#[allow(unused)]
async fn udev_parse() -> Result<(), Box<dyn std::error::Error>>
{
    // println!("udev init");
    // let builder = MonitorBuilder::new()
    //     .expect("Couldn't create builder")
    //     .match_subsystem_devtype("usb", "usb_device")
    //     .expect("Failed to add filter for USB devices");
    // let monitor: AsyncMonitorSocket = builder
    //     .listen()
    //     .expect("Couldn't create MonitorSocket")
    //     .try_into()
    //     .expect("Couldn't create AsyncMonitorSocket");
    // monitor.for_each(|event| {
    //     if let Ok(event) = event {
    //         println!(
    //             "Hotplug event: {}: {}",
    //             event.event_type(),
    //             event.device().syspath().display()
    //         );
    //     }
    //     ready(())
    // }).await;
    // println!("udev free");

    println!("udev enumerator init");
    let mut enumerator = udev::Enumerator::new()?;
    enumerator.match_subsystem("usb")?;
    enumerator.match_property("ID_SIGROK", "1");

    for device in enumerator.scan_devices()? {
        // println!("{:#?}", device.syspath());
        // println!("{:#?}", device.devpath());
        println!();
        println!("{:#?}", device);

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }
    println!("udev enumerator free");

    Ok(())
}

use serialport::{available_ports, SerialPortType};

#[allow(unused)]
async fn serial_parse() -> Result<(), Box<dyn std::error::Error>>
{
    match available_ports() {
        Ok(ports) => {
            println!("ports count {}", ports.len());
            for p in ports {
                println!("ports name {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("Type: USB info {:?}", info);
                        println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                        println!(
                            "     Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "      Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "           Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                        #[cfg(feature = "usbportinfo-interface")]
                        println!(
                            "         Interface: {}",
                            info.interface
                                .as_ref()
                                .map_or("".to_string(), |x| format!("{:02x}", *x))
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("Type: BluetoothPort");
                    }
                    SerialPortType::PciPort => {
                        println!("Type: PciPort");
                    }
                    SerialPortType::Unknown => {
                        println!("Type: Unknown");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}

// async fn websocket_init() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

//     // build our application with some routes
//     let app = Router::new()
//         .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
//         .route("/ws", get(ws_handler))
//         // logging so we can see whats going on
//         .layer(
//             TraceLayer::new_for_http()
//                 .make_span_with(DefaultMakeSpan::default().include_headers(true)),
//         );

//     // run it with hyper
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//         .await
//         .unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(
//         listener,
//         app.into_make_service_with_connect_info::<SocketAddr>(),
//     )
//     .await
//     .unwrap();
// }
