use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use clap::Parser;
// use futures_util::future::ready;
// use futures_util::stream::StreamExt;
use serde::Deserialize;
// use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

pub struct Canbus {
    pub baudrate: u32,
}
pub struct Current {
    pub crnt: u32,
}
pub struct Voltage {
    pub volt: u32,
}
pub enum Caliber {
    Canbus(Canbus),
    Current(Current),
    Voltage(Voltage),
}

pub struct Utensil {
    pub calibers: Vec<Caliber>,
}

pub struct Toolkit {
    pub name: String,
    // device: Device,
    pub calibers: Vec<Utensil>,
}

pub struct Snooper {
    pub name: String,
    pub toolkits: Vec<Toolkit>,
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    println!("cli: {:?}", cli);

    let innate_path = cli.fluid_innate.unwrap_or(cli.solid_innate.unwrap_or(Path::new("./utils/innate.json").to_path_buf()));
    let innate_file = File::open(innate_path)?;
    let innate_reader = BufReader::new(innate_file);
    let innate: Innate = serde_json::from_reader(innate_reader)?;
    println!("innate {:?}", innate);

    // for dev in nusb::list_devices().unwrap() {
    //     println!("dev {:#?}", dev);
    // }

    // let builder = MonitorBuilder::new()
    //     .expect("Couldn't create builder")
    //     .match_subsystem_devtype("usb", "usb_device")
    //     .expect("Failed to add filter for USB devices");

    // let monitor: AsyncMonitorSocket = builder
    //     .listen()
    //     .expect("Couldn't create MonitorSocket")
    //     .try_into()
    //     .expect("Couldn't create AsyncMonitorSocket");
    // monitor.for_each(|event| {
    //     if let Ok(event) = event {
    //         println!(
    //             "Hotplug event: {}: {}",
    //             event.event_type(),
    //             event.device().syspath().display()
    //         );
    //     }
    //     ready(())
    // }).await;
    // println!("udev return");

    let mut enumerator = udev::Enumerator::new()?;
    // enumerator.match_subsystem("usb")?;

    for device in enumerator.scan_devices()? {
        // println!("{:#?}", device.syspath());
        // println!("{:#?}", device.devpath());
        println!();
        println!("{:#?}", device);

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}
