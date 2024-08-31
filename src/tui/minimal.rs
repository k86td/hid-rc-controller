use super::{utils::print_middle_row, widgets::steering_wheel_chart};
use crossterm::{
    event::{read, Event, KeyCode},
    queue,
    terminal::{self, Clear},
};
use hidapi::HidApi;
use std::{
    fmt::Display,
    io::{self, Stdout},
    time::Duration,
};

// TODO: move this somewhere else
pub struct SteeringWheelData {
    steering_angle: u8,
    gas_pedal: DualPrecisionPedal,
    brake_pedal: DualPrecisionPedal,
}

pub struct DualPrecisionPedal {
    precise: u8,
    general: u8,
}

impl From<&[u8]> for DualPrecisionPedal {
    fn from(value: &[u8]) -> Self {
        Self {
            precise: value[0],
            general: value[1],
        }
    }
}

impl From<[u8; 32]> for SteeringWheelData {
    fn from(value: [u8; 32]) -> Self {
        Self {
            steering_angle: value[2],
            gas_pedal: value[7..=8].into(),
            brake_pedal: value[3..=4].into(),
        }
    }
}

impl DualPrecisionPedal {
    fn convert_dual_precision(precise: u8, general: u8, max_value: usize, invert: bool) -> usize {
        if invert {
            ((general as i16 * 255) + precise as i16 - max_value as i16)
                .abs()
                .try_into()
                .unwrap()
        } else {
            (general * 255 + precise).into()
        }
    }
}

impl Display for DualPrecisionPedal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &DualPrecisionPedal::convert_dual_precision(self.precise, self.general, 1020, true)
                .to_string(),
        )
    }
}

pub fn app(api: &mut HidApi, stdout: &mut Stdout) -> io::Result<()> {
    if let Some(dev) = api.device_list().next() {
        if let Ok(wheel) = dev.open_device(api) {
            let mut buf: [u8; 32] = [0; 32];

            loop {
                wheel.read(&mut buf).unwrap();
                let wheel_data: SteeringWheelData = buf.into();
                if crossterm::event::poll(Duration::from_nanos(10))? {
                    let event = read()?;
                    if event == Event::Key(KeyCode::Char('q').into()) {
                        break;
                    }
                } else {
                    queue!(stdout, Clear(terminal::ClearType::FromCursorUp),).unwrap();

                    // print_middle_row(&format!("{:?}", buf), 1, stdout);
                    print_middle_row(&wheel_data.steering_angle.to_string(), 2, stdout);
                    print_middle_row(
                        &steering_wheel_chart(
                            wheel_data.steering_angle.into(),
                            (terminal::size().unwrap().1).into(),
                            255,
                            "#",
                        ),
                        3,
                        stdout,
                    );
                    print_middle_row(
                        &format!(
                            "gas: {} | brake: {}",
                            wheel_data.gas_pedal, wheel_data.brake_pedal
                        ),
                        4,
                        stdout,
                    );
                }
            }
        }
    }

    Ok(())
}
