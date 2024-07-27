

// 上电初始化一次的参数 'static
#[derive(Deserialize, Debug)]
pub struct Innate {
	// 公共信息存放的位置，如数据库文件assets.db
    pub fibase_shrine: PathBuf,
    pub sibase_shrine: PathBuf,

	// 参数存放的位置
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

    pub id: u16,
    pub name: String,
}

#[derive(clap::Parser, Debug)]
struct Cli {
    #[arg(short = 'f', long = "fibase_innate")]
    fibase_innate: Option<PathBuf>,
    #[arg(short = 's', long = "sibase_innate")]
    sibase_innate: Option<PathBuf>,
}

static INNATE: std:sync::OnceLock<Innate> = std::sync::OnceLock::new();

pub(crate) innate()
{
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    let innate_path = cli.fibase_innate.unwrap_or(cli.sibase_innate.unwrap_or(Path::new("./scenes/dossier/utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

	INNATE.set(innate);
	//let _ = INNATE.set(innate);

}
