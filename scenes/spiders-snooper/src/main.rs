use std::path::PathBuf;
use clap::Parser;
use sigrok::config::{config_items, Configurable};
use sigrok::data::{Datafeed, Logic};
use sigrok::{Session, Sigrok};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    let ctx = Sigrok::new()?;
    let ses = Session::new(&ctx)?;

    let driver = ctx.drivers().into_iter().find(|x| x.name() == "demo").unwrap();

    let driver = driver.init()?;

    for device in driver.scan(None)? {
        ses.add_device(&device)?;
        device.config_set(config_items::LimitSamples, &64)?;

        if let Some(group) = device.channel_groups().get(0) {
            group.config_set(config_items::PatternMode, "sigrok")?;
        }

        device.config_set(config_items::SampleRate, &1_000_000)?;
    }

    ses.start(None, |_, data| match data {
        Datafeed::Logic(Logic { unit_size, data }) => {
            let _ = unit_size;
            for byte in data {
                println!("{}", format!("{:08b}", byte).replace("0", " "));
            }
        }
        _ => {}
    })?;

    Ok(())
}
