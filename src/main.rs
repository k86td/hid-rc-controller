// extern crate hidapi;

pub mod drivers;
pub mod tui;
pub mod utils;

use drivers::{linux_i2c::ExtendedLinuxI2CDevice, mcp4725::MCP4725};
use hidapi::HidApi;
use linux_embedded_hal::i2cdev::linux::LinuxI2CDevice;
use std::io;
use tui::{init, minimal::app, restore};

fn main() -> io::Result<()> {
    if let Ok(mut api) = HidApi::new() {
        // let gas_i2c = ExtendedLinuxI2CDevice {
        //     dev: LinuxI2CDevice::new("/dev/i2c-1", 0x60).unwrap(),
        // };
        // let brake_i2c = ExtendedLinuxI2CDevice {
        //     dev: LinuxI2CDevice::new("/dev/i2c-1", 0x61).unwrap(),
        // };
        //
        // let mut gas_dac = MCP4725::new(gas_i2c, 0x60);
        // let mut steering_dac = MCP4725::new(brake_i2c, 0x61);

        let mut stdout = io::stdout();
        init(&mut stdout)?;

        if let Err(e) = app(&mut api, &mut stdout) {
            println!("Error: {:?}\r", e);
        }

        restore(&mut stdout)?;
    }

    Ok(())
}
