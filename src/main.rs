extern crate hidapi;

mod tui;
use hidapi::HidApi;
use ratatui::backend::CrosstermBackend;
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

const BUF_SIZE: usize = 32;
const WAIT_TIME: Duration = Duration::new(0, 50);

fn main() -> io::Result<()> {
    if let Ok(api) = HidApi::new() {
        let mut terminal = tui::init()?;
        let app = tui::app::App::default().run(&mut terminal);

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

    Ok(())
}
