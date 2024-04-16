use std::path::{PathBuf};
use clap::Parser;
use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

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

pub struct Toolkit {
    pub name: String,
    // device: Device,
    pub calibers: Vec<Caliber>,
}

pub struct Snooper {
    pub name: String,
    pub toolkits: Vec<Toolkit>,
}

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

    // for dev in nusb::list_devices().unwrap() {
    //     println!("dev {:#?}", dev);
    // }

    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem_devtype("usb", "usb_device")
        .expect("Failed to add filter for USB devices");

    let monitor: AsyncMonitorSocket = builder
        .listen()
        .expect("Couldn't create MonitorSocket")
        .try_into()
        .expect("Couldn't create AsyncMonitorSocket");
    monitor.for_each(|event| {
        if let Ok(event) = event {
            println!(
                "Hotplug event: {}: {}",
                event.event_type(),
                event.device().syspath().display()
            );
        }
        ready(())
    }).await;
    println!("udev return");

    Ok(())
}
