//! This example shows how to tell Android its connected to an Android Auto Accessory
use std::error::Error;
use std::time::Duration;

use rusb::DeviceList;

use aoa::{AccessoryDevice, AccessoryHandle, AccessoryStrings};

fn main() {
    check_devices().unwrap()
}

fn check_devices() -> Result<(), Box<dyn Error>> {
    let strings = AccessoryStrings::new(
        "Android",
        "Android Auto",
        "Android Auto",
        "2.0.1",
        "https://play.google.com/store/apps/details?id=com.google.android.projection.gearhead",
        "HU-AAAAAA001",
    )?;

    let timeout = Duration::from_secs(1);
    for device in DeviceList::new()?.iter() {
        // Check if the device is already in accessory mode
        if device.in_accessory_mode()? {
            // Its possible that the OS or Device doesn't allow opening a handle.
            let mut handle = match device.open() {
                Ok(device) => device,
                Err(_) => continue,
            };

            // If the device is already in accessory mode then we can request the protocol version
            if let Ok(protocol) = handle.get_protocol(timeout) {
                let endpoints = device.find_endpoints()?;
                println!(
                    "Found Accessory - Device: {:?}, Protocol: {}, Endpoints: {:?}",
                    device, protocol, endpoints
                )
            }
            continue;
        }

        // Its possible that the OS or Device doesn't allow opening a handle.
        let mut handle = match device.open() {
            Ok(device) => device,
            Err(_) => continue,
        };

        if handle.claim_interface(0).is_err() {
            continue;
        }

        match handle.start_accessory(&strings, timeout) {
            Ok(protocol) => {
                let endpoints = device.find_endpoints()?;
                println!(
                    "Started Accessory - Device: {:?}, Protocol: {}, Endpoints: {:?}",
                    device, protocol, endpoints
                );
            }
            Err(err) => {
                eprintln!(
                    "Accessory mode not supported - Device: {:?}, {:?}",
                    device, err
                );
                continue;
            }
        }

        handle.release_interface(0)?;
    }

    Ok(())
}
