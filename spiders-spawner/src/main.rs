// use clap::Parser;

use std::path::PathBuf;
use clap::Parser;
#[allow(unused_imports)]
use serde_json::Value;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short='s', long="solid")]
    solid_file: Option<PathBuf>,
    #[arg(short='f', long="fluid")]
    fluid_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("cli: {:?}", cli);
}
