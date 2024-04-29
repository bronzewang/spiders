use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
use serde::Deserialize;

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

// 上电初始化一次的参数 'static
#[derive(Deserialize, Debug)]
pub struct Innate {
    pub shrine: PathBuf,        //spider数据库保存路径  assets.db

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

    pub surname: String,
    pub version: String,
    pub menages: Vec<Menage>,
}

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
