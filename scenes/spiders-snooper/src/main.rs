use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
use serde::Deserialize;
use sigrok::config::{config_items, Configurable};
use sigrok::data::{Datafeed, Logic};
use sigrok::{Session, Sigrok};

// 上电初始化一次的参数 'static
#[derive(Deserialize, Debug)]
pub struct Innate {
    pub fibase_valver: PathBuf,
    pub sibase_valver: PathBuf,
    pub fibase_snaper: PathBuf,
    pub sibase_snaper: PathBuf,
    pub sibase_larder: PathBuf,
    pub fibase_larder: PathBuf,
    pub sxmass_larder: PathBuf,
    pub fxmass_larder: PathBuf,
    pub sxplug_larder: PathBuf,
    pub fxplug_larder: PathBuf,
}

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short = 's', long = "solid_innate")]
    solid_innate: Option<PathBuf>,
    #[arg(short = 'f', long = "fluid_innate")]
    fluid_innate: Option<PathBuf>,
}

// static INNATE: Innate = static_cell();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    let innate_path = cli.solid_innate.unwrap_or(cli.fluid_innate.unwrap_or(Path::new("./utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

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
