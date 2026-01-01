#![allow(unused_imports)]
use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use serde::Deserialize;

pub mod greeter {
	include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}

pub use greeter::*;

// 上电初始化一次的参数 'static
// $(path)/${surname}/${menage}/fibase_innate.json
// $(path)/${surname}/${menage}/fibase_shrine/
// $(path)/${surname}/${menage}/sibase_shrine/

#[derive(Deserialize, Debug)]
pub struct Innate {
	// 先天特性文件存放的位置
	pub fibase_innate: PathBuf,
	pub sibase_innate: PathBuf,

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

    pub surname: String,
    pub version: String,
    pub menages: Vec<Menage>
}
