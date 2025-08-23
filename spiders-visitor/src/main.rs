use std::error::Error;
use std::time::Instant;
use std::time::Duration;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 100)]
    rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("test");
    let rate = Duration::from_millis(cli.rate);
    run(rate)
}

fn run(_rate: Duration) -> Result<(), Box<dyn Error>> {
    let mut _last_tick = Instant::now();
    // loop {
    //     println!("test");
    // }
    Ok(())
}
