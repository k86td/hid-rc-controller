extern crate hidapi;

mod tui;
use hidapi::HidApi;
use ratatui::backend::CrosstermBackend;
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

fn main() -> io::Result<()> {
    if let Ok(api) = HidApi::new() {
        let mut terminal = tui::init()?;
        let app = tui::app::App::default().run(&mut terminal);

        tui::restore()?;

        // if let Some(dev) = api.device_list().next() {
        //     println!(
        //         "dev:{:?}, manufacturer:{:?}",
        //         dev,
        //         dev.manufacturer_string(),
        //     );
        //
        //     if let Ok(wheel) = dev.open_device(&api) {
        //         let mut buf: [u8; 32] = [0; 32];
        //         loop {
        //             wheel.read(&mut buf).unwrap();
        //             dbg!(&buf[1..3]);
        //         }
        //     }
        // }
    }

    Ok(())
}
