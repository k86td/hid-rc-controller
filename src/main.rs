extern crate hidapi;

use hidapi::HidApi;
use std::{thread::sleep, time::Duration};

const BUF_SIZE: usize = 32;
const WAIT_TIME: Duration = Duration::new(0, 50);

fn main() {
    if let Ok(api) = HidApi::new() {
        if let Some(dev) = api.device_list().next() {
            println!(
                "dev:{:?}, manufacturer:{:?}",
                dev,
                dev.manufacturer_string(),
            );

            if let Ok(wheel) = dev.open_device(&api) {
                let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
                loop {
                    wheel
                        .read_timeout(&mut buf, WAIT_TIME.as_millis() as i32)
                        .unwrap();
                    dbg!(buf);
                    sleep(WAIT_TIME);
                }
            }
        }
    }
}
