// extern crate hidapi;

pub mod drivers;
pub mod tui;
pub mod utils;

use hidapi::HidApi;
use std::io;
use tui::{init, minimal::app, restore};

fn main() -> io::Result<()> {
    if let Ok(mut api) = HidApi::new() {
        let mut stdout = io::stdout();
        init(&mut stdout)?;

        if let Err(e) = app(&mut api, &mut stdout) {
            println!("Error: {:?}\r", e);
        }

        restore(&mut stdout)?;
    }

    Ok(())
}
