// mod app;
// use app::*;

// mod book;
// use book::*;

mod node;
use node::*;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    // let conn = Database::connect("sqlite::memory:")
    //     .await
    //     .expect("Database connection failed");

    mount_to_body(|| {
        view! {
            // <BookList />
            <Node />
        }
    })
}
