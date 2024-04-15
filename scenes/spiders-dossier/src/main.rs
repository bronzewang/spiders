use std::path::{PathBuf};
use clap::Parser;

pub struct Canbus {
    pub baudrate: u32,
}
pub struct Current {
    pub crnt: u32,
}
pub struct Voltage {
    pub volt: u32,
}
pub enum Caliber {
    Canbus(Canbus),
    Current(Current),
    Voltage(Voltage),
}

pub struct Toolkit {
    pub name: String,
    // device: Device,
    pub calibers: Vec<Caliber>,
}

pub struct Snooper {
    pub name: String,
    pub toolkits: Vec<Toolkit>,
}
#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    inner_path: Option<PathBuf>,
    #[arg(short, long)]
    xmass_path: Option<PathBuf>,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    println!("cli: {:?}", cli);

    Ok(())
}
