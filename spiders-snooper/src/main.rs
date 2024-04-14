use std::path::PathBuf;
use clap::Parser;

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
