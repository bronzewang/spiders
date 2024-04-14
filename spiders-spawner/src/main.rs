// use clap::Parser;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use clap::Parser;
//use futures::io::BufReader;
use serde::Deserialize;
// #[allow(unused_imports)]
// use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct RoleMaster {
    pub binary: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Menage {
    pub name: String,
    pub step: u8,
    pub master: RoleMaster,
}

#[derive(Deserialize, Debug)]
pub struct SolidConfig {
    pub surname: String,
    pub version: String,
    pub menages: Vec<Menage>,
}

#[derive(Deserialize, Debug)]
pub struct FluidConfig {
    pub menages: Vec<Menage>,
}

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short='s', long="solid")]
    solid_file: Option<PathBuf>,
    #[arg(short='f', long="fluid")]
    fluid_file: Option<PathBuf>,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("cli: {:?}", cli);

    // let solid_file = if Same(solid) = cli.solid_file {
    //     solid
    // } else {
    //     PathBuf::new()
    // }

    let file = File::open("./config/solid.json")?;
    let reader = BufReader::new(file);
    let solid: SolidConfig = serde_json::from_reader(reader)?;
    println!("solid config {:?}", solid);

    let file = File::open("./config/fluid.json")?;
    let reader = BufReader::new(file);
    let fluid: FluidConfig = serde_json::from_reader(reader)?;
    println!("fluid config {:?}", fluid);

    Ok(())
}
