use super::{utils::print_middle_row, widgets::steering_wheel_chart};
use crossterm::{
    event::{read, Event, KeyCode},
    queue,
    terminal::{self, Clear},
};
use hidapi::HidApi;
use std::{
    io::{self, Stdout},
    time::Duration,
};

pub fn app(api: &mut HidApi, stdout: &mut Stdout) -> io::Result<()> {
    if let Some(dev) = api.device_list().next() {
        if let Ok(wheel) = dev.open_device(api) {
            let mut buf: [u8; 32] = [0; 32];

            loop {
                wheel.read(&mut buf).unwrap();
                if crossterm::event::poll(Duration::from_nanos(10))? {
                    let event = read()?;
                    if event == Event::Key(KeyCode::Char('q').into()) {
                        break;
                    }
                } else {
                    queue!(stdout, Clear(terminal::ClearType::FromCursorUp),).unwrap();

                    print_middle_row(&format!("{:?}", terminal::size()), 1, stdout);
                    print_middle_row(&buf[2].to_string(), 2, stdout);
                    print_middle_row(
                        &steering_wheel_chart(
                            buf[2].into(),
                            (terminal::size().unwrap().1).into(),
                            255,
                            "#",
                        ),
                        3,
                        stdout,
                    );
                }
            }
        }
    }

    Ok(())
}
