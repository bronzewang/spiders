use anyhow::Result;
use console_subscriber::ConsoleLayer;
use opentelemetry::{InstrumentationScope, global, trace::TracerProvider};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    logs::SdkLoggerProvider,
    metrics::{PeriodicReader, SdkMeterProvider},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
};
use opentelemetry_system_metrics::init_process_observer;
use tarpc::tokio_serde::formats::Bincode;
use tokio::{
    net::UnixStream,
    time::{Duration, interval},
};
use tokio_util::codec::LengthDelimitedCodec;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use std::path::PathBuf;

#[tarpc::service]
trait DossiserService {
    async fn handshake();
}

struct OtelGuard {
    logger_provider: SdkLoggerProvider,
    meter_provider: SdkMeterProvider,
    tracer_provider: SdkTracerProvider,
}
impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(e) = self.logger_provider.shutdown() {
            eprintln!("logger_provider shutdown fail error {e}");
        }
        if let Err(e) = self.meter_provider.shutdown() {
            eprintln!("meter_provider shutdown fail error {e}");
        }
        if let Err(e) = self.tracer_provider.shutdown() {
            eprintln!("tracer_provider shutdown fail error {e}");
        }
    }
}

fn init_tracing_subscriber() -> OtelGuard {
    let logger_provider = init_logger_provider();
    let meter_provider = init_meter_provider();
    let tracer_provider = init_tracer_provider();

    let otel_filter = EnvFilter::new("debug")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider).with_filter(otel_filter);

    let term_filter = EnvFilter::new("warn");
    let term_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_filter(term_filter);

    let console_layzer = ConsoleLayer::builder()
        .retention(ConsoleLayer::DEFAULT_RETENTION)
        .server_addr(([127, 0, 0, 1], 6670))
        .spawn();

    let tracer = tracer_provider.tracer("snooper");
    tracing_subscriber::registry()
        .with(console_layzer)
        .with(otel_layer)
        .with(term_layer)
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    let scope = InstrumentationScope::builder("dossier")
        .with_version("0.1")
        .build();
    let meter = global::meter_with_scope(scope);
    tokio::spawn(init_process_observer(meter));

    OtelGuard {
        logger_provider,
        meter_provider,
        tracer_provider,
    }
}

const TONIC_ENDPOINT: &str = "http://192.168.3.47:4317";

fn resource() -> Resource {
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .build()
}

fn init_logger_provider() -> SdkLoggerProvider {
    let otel_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(TONIC_ENDPOINT)
        .build()
        .expect("init log exporter fail");
    let term_exporter = opentelemetry_stdout::LogExporter::default();

    SdkLoggerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(otel_exporter)
        .with_batch_exporter(term_exporter)
        .build()
}

fn init_meter_provider() -> SdkMeterProvider {
    let otel_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(TONIC_ENDPOINT)
        .build()
        .expect("metric exporter build fail");
    let otel_reader = PeriodicReader::builder(otel_exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    let term_exportor = opentelemetry_stdout::MetricExporter::builder().build();
    let term_reader = PeriodicReader::builder(term_exportor)
        .with_interval(Duration::from_secs(5))
        .build();

    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource())
        .with_reader(otel_reader)
        .with_reader(term_reader)
        .build();
    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

fn init_tracer_provider() -> SdkTracerProvider {
    let otel_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(TONIC_ENDPOINT)
        .build()
        .expect("span exporter build fail");
    let term_exporter = opentelemetry_stdout::SpanExporter::default();

    let otel_provider = SdkTracerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(otel_exporter)
        .with_batch_exporter(term_exporter)
        .with_id_generator(RandomIdGenerator::default())
        .with_sampler(Sampler::AlwaysOn)
        // .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
        //     1.0,
        // ))))
        .build();
    global::set_tracer_provider(otel_provider.clone());

    otel_provider
}

#[tokio::main]
async fn main() -> Result<()> {
    let _otel_guard = init_tracing_subscriber();

    let connect_addr = PathBuf::from("/tmp")
        .join("spiders")
        .join("spiders-dossier")
        .with_extension("sock");
    let conn = UnixStream::connect(connect_addr).await?;

    let transport = tarpc::serde_transport::new(
        LengthDelimitedCodec::builder().new_framed(conn),
        Bincode::default(),
    );
    let client = DossiserServiceClient::new(Default::default(), transport).spawn();
    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;
        client.handshake(tarpc::context::current()).await?
    }
}

// use clap::Parser;
// use serde::Deserialize;
// use std::{
//     fs::File,
//     io::BufReader,
//     path::{Path, PathBuf},
// };
// // use sigrok::config::{config_items, Configurable};
// // use sigrok::data::{Datafeed, Logic};
// // use sigrok::{Session, Sigrok};

// pub mod greeter {
//     include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
// }

// pub use greeter::*;

// // 上电初始化一次的参数 'static
// #[derive(Deserialize, Debug)]
// pub struct Innate {
//     // 公共信息存放的位置，如数据库文件assets.db
//     pub fibase_shrine: PathBuf,
//     pub sibase_shrine: PathBuf,

//     // 参数存放的位置
//     pub fibase_valver: PathBuf,
//     pub sibase_valver: PathBuf,

//     pub fibase_snaper: PathBuf,
//     pub sibase_snaper: PathBuf,

//     pub sibase_larder: PathBuf,
//     pub fibase_larder: PathBuf,
//     pub sxmass_larder: PathBuf,
//     pub fxmass_larder: PathBuf,
//     pub sxplug_larder: PathBuf,
//     pub fxplug_larder: PathBuf,

//     pub name: String,
// }
// // static INNATE: Innate = static_cell();

// #[derive(clap::Parser, Debug)]
// struct Cli {
//     #[arg(short = 's', long = "sibase_innate")]
//     sibase_innate: Option<PathBuf>,
//     #[arg(short = 'f', long = "fibase_innate")]
//     fibase_innate: Option<PathBuf>,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let cli = Cli::parse();
//     println!("cli: {:?}", cli);

//     let innate_path = cli.fibase_innate.unwrap_or(
//         cli.sibase_innate
//             .unwrap_or(Path::new("./utils/innate.json").to_path_buf()),
//     );
//     let innate_file = File::open(innate_path)?;
//     let innate_reader = BufReader::new(innate_file);
//     let innate: Innate = serde_json::from_reader(innate_reader)?;
//     println!("innate {:?}", innate);

//     // let ctx = Sigrok::new()?;
//     // let ses = Session::new(&ctx)?;

//     // let driver = ctx.drivers().into_iter().find(|x| x.name() == "demo").unwrap();

//     // let driver = driver.init()?;

//     // for device in driver.scan(None)? {
//     //     ses.add_device(&device)?;
//     //     device.config_set(config_items::LimitSamples, &64)?;

//     //     if let Some(group) = device.channel_groups().get(0) {
//     //         group.config_set(config_items::PatternMode, "sigrok")?;
//     //     }

//     //     device.config_set(config_items::SampleRate, &1_000_000)?;
//     // }

//     // ses.start(None, |_, data| match data {
//     //     Datafeed::Logic(Logic { unit_size, data }) => {
//     //         let _ = unit_size;
//     //         for byte in data {
//     //             println!("{}", format!("{:08b}", byte).replace("0", " "));
//     //         }
//     //     }
//     //     _ => {}
//     // })?;

//     Ok(())
// }
