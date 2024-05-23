use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
// use futures_util::future::ready;
// use futures_util::stream::StreamExt;
use serde::Deserialize;
// use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};
use axum::{response::Html, routing::get, Router};
use log::{error, Level};
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::{Config, LoggerProvider};
use opentelemetry_sdk::Resource;

use spiders_scheme::SPIDERS_WEB_PORT_BASE;

pub mod greeter {
	include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}

pub use greeter::*;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    let innate_path = cli.fibase_innate.unwrap_or(cli.sibase_innate.unwrap_or(Path::new("./utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

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

    // build our application with a route
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",
                                                         SPIDERS_WEB_PORT_BASE+innate.id))
        .await
        .unwrap();
    // run it
    error!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World! I am Spiders.</h1>")
}

// fn udev_parse()
// {
//     let builder = MonitorBuilder::new()
//         .expect("Couldn't create builder")
//         .match_subsystem_devtype("usb", "usb_device")
//         .expect("Failed to add filter for USB devices");

//     let monitor: AsyncMonitorSocket = builder
//         .listen()
//         .expect("Couldn't create MonitorSocket")
//         .try_into()
//         .expect("Couldn't create AsyncMonitorSocket");
//     monitor.for_each(|event| {
//         if let Ok(event) = event {
//             println!(
//                 "Hotplug event: {}: {}",
//                 event.event_type(),
//                 event.device().syspath().display()
//             );
//         }
//         ready(())
//     }).await;
//     println!("udev return");

//     let mut enumerator = udev::Enumerator::new()?;
//     // enumerator.match_subsystem("usb")?;

//     for device in enumerator.scan_devices()? {
//         // println!("{:#?}", device.syspath());
//         // println!("{:#?}", device.devpath());
//         println!();
//         println!("{:#?}", device);

//         println!("  [properties]");
//         for property in device.properties() {
//             println!("    - {:?} {:?}", property.name(), property.value());
//         }

//         println!("  [attributes]");
//         for attribute in device.attributes() {
//             println!("    - {:?} {:?}", attribute.name(), attribute.value());
//         }
//     }

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
