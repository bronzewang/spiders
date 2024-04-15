use std::path::PathBuf;
use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(long = "fibase")]
    fluid_ibase: Option<PathBuf>,
    #[arg(long = "fxmass")]
    fluid_xmass: Option<PathBuf>,
    #[arg(long = "fxplug")]
    fluid_xplug: Option<PathBuf>,

    #[arg(long = "sibase")]
    solid_ibase: Option<PathBuf>,
    #[arg(long = "sxmass")]
    solid_xmass: Option<PathBuf>,
    #[arg(long = "sxplug")]
    solid_xplug: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    Ok(())
}
