// 采集udev和serial设备信息，保存在一个vec中

use std::error::Error;

// 驱动方式
#[deny(dead_code)]
pub enum Means {
	Serial,
	Sigrok,
}

async fn toolkit_init() -> Result<(), Box<dyn Error>>
{
	// use futures_util::future::ready;
	// use futures_util::stream::StreamExt;
	// use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

	// // 创建sigrok事件监控任务
    // let builder = MonitorBuilder::new()
	// 	.expect("Couldn't create builder")
	// 	.match_subsystem_devtype("usb", "usb_device")
	// 	.expect("Failed to add filter for USB devices");
	// let monitor: AsyncMonitorSocket = builder
	// 	.listen()
	// 	.expect("Couldn't create MonitorSocket")
	// 	.try_into()
	// 	.expect("Couldn't create AsyncMonitorSocket");
	// monitor.for_each(toolkit_sigrok_event).await;

	// // 创建sigrok定期采集任务
	// tokio::spawn(toolkit_sigrok_glean);

	// // 创建serial事件监控任务
    // let builder = MonitorBuilder::new()
	// 	.expect("Couldn't create builder")
	// 	.match_subsystem_devtype("usb", "usb_device")
	// 	.expect("Failed to add filter for USB devices");
	// let monitor: AsyncMonitorSocket = builder
	// 	.listen()
	// 	.expect("Couldn't create MonitorSocket")
	// 	.try_into()
	// 	.expect("Couldn't create AsyncMonitorSocket");
	// monitor.for_each(toolkit_sigrok_event).await;

	// // 创建serial定期采集任务
	// tokio::spawn(toolkit_serial_glean);

	Ok(())
}

// async fn toolkit_sigrok_event(event) -> Result<(), Box<dyn Error>>
// {
// 	if let Ok(event) = event {
// 		println!(
// 			"Hotplug event: {}: {}",
// 			event.event_type(),
// 			event.device().syspath().display()
// 		);
// 	}
// 	ready(())
// }

// async fn toolkit_sigrok_glean() -> Result<(), Box<dyn Error>>
// {
// 	loop {
// 		println!("udev enumerator init");
// 		let mut enumerator = udev::Enumerator::new()?;
// 		enumerator.match_subsystem("usb")?;
// 		enumerator.match_property("ID_SIGROK", "1");

// 		for device in enumerator.scan_devices()? {
// 		    // println!("{:#?}", device.syspath());
// 		    // println!("{:#?}", device.devpath());
// 		    println!();
// 		    println!("{:#?}", device);

// 		    println!("  [properties]");
// 		    for property in device.properties() {
// 		        println!("    - {:?} {:?}", property.name(), property.value());
// 		    }

// 		    println!("  [attributes]");
// 		    for attribute in device.attributes() {
// 		        println!("    - {:?} {:?}", attribute.name(), attribute.value());
// 		    }
// 		}
// 		println!("udev enumerator free");
// 	}

//     Ok(())
// }

// async fn toolkit_serial_event(event) -> Result<(), Box<dyn Error>>
// {
// 	if let Ok(event) = event {
// 		println!(
// 			"Hotplug event: {}: {}",
// 			event.event_type(),
// 			event.device().syspath().display()
// 		);
// 	}
// 	ready(())
// }

// async fn toolkit_serial_glean() -> Result<(), Box<dyn Error>>
// {
// 	use serialport::{available_ports, SerialPortType};

//     match available_ports() {
//         Ok(ports) => {
//             println!("ports count {}", ports.len());
//             for p in ports {
//                 println!("ports name {}", p.port_name);
//                 match p.port_type {
//                     SerialPortType::UsbPort(info) => {
//                         println!("Type: USB info {:?}", info);
//                         println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
//                         println!(
//                             "     Serial Number: {}",
//                             info.serial_number.as_ref().map_or("", String::as_str)
//                         );
//                         println!(
//                             "      Manufacturer: {}",
//                             info.manufacturer.as_ref().map_or("", String::as_str)
//                         );
//                         println!(
//                             "           Product: {}",
//                             info.product.as_ref().map_or("", String::as_str)
//                         );
//                         #[cfg(feature = "usbportinfo-interface")]
//                         println!(
//                             "         Interface: {}",
//                             info.interface
//                                 .as_ref()
//                                 .map_or("".to_string(), |x| format!("{:02x}", *x))
//                         );
//                     }
//                     SerialPortType::BluetoothPort => {
//                         println!("Type: BluetoothPort");
//                     }
//                     SerialPortType::PciPort => {
//                         println!("Type: PciPort");
//                     }
//                     SerialPortType::Unknown => {
//                         println!("Type: Unknown");
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("{:?}", e);
//         }
//     }

//     Ok(())
// }
