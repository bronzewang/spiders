use axum::{response::Html, extract::State, routing::get, Router};
use log::{error, Level};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::{logs::{Config, LoggerProvider}, metrics::SdkMeterProvider};
use prometheus::{Registry};
use std::sync::Arc;
use std::error::Error;
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider as _, Unit},
    KeyValue,
};

#[derive(Debug)]
struct AppState {
	// toolkit: Vec<Toolkit>,
	// snooper: Vec<Snooper>,

    registry: Registry,
    http_counter: Counter<u64>,
    http_body_gauge: Histogram<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use spiders_dossier::{app::*, fallback::file_and_error_handler};

	// toolkit_init().await?;

    let exporter = opentelemetry_stdout::LogExporterBuilder::default().build();
    let logger_provider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(opentelemetry_sdk::Resource::new(vec![KeyValue::new(
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
		// toolkit: Vec::new(),
		// snooper: Vec::new(),
        registry: registry,
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
    // let app = Router::new().route("/", get(handler)).with_state(state);
    // let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",
    //                                                      SPIDERS_WEB_PORT_BASE+innate.id))
    //     .await
    //     .unwrap();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

	// use std::net::SocketAddr;
    // let addr: SocketAddr = conf.leptos_options.site_addr.parse();
	// addr.set_port(addr.port()+innate.id);

    let app = Router::new()
        .route("/api", get(handler)).with_state(state)
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

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
