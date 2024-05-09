use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
use tribers_spawner::{Innate};

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short = 's', long = "solid_innate")]
    solid_innate: Option<PathBuf>,
    #[arg(short = 'f', long = "fluid_innate")]
    fluid_innate: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("cli: {:?}", cli);

    let innate_path = cli.fluid_innate.unwrap_or(cli.solid_innate.unwrap_or(Path::new("./utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

    Ok(())
}
