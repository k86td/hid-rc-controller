use crate::{
    drivers::{linux_i2c::ExtendedLinuxI2CDevice, mcp4725::MCP4725},
    utils::{convert_value_to_dac, value_to_voltage},
};

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

// TODO: move this somewhere else
pub struct SteeringWheelData {
    steering_angle: u8,
    gas_pedal: DualPrecisionPedal,
    brake_pedal: DualPrecisionPedal,
}

impl SteeringWheelData {
    pub fn gas(&self) -> usize {
        let pedal = &self.gas_pedal;
        DualPrecisionPedal::convert_dual_precision(pedal.precise, pedal.general, 1020, true)
    }

    pub fn brake(&self) -> usize {
        let pedal = &self.brake_pedal;
        DualPrecisionPedal::convert_dual_precision(pedal.precise, pedal.general, 1020, false)
    }
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
            gas_pedal: value[5..=6].into(),
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
            general as usize * 255 + precise as usize
        }
    }
}

pub fn app(
    api: &mut HidApi,
    stdout: &mut Stdout,
    // gas_dac: &mut MCP4725<ExtendedLinuxI2CDevice>,
    // steering_dac: &mut MCP4725<ExtendedLinuxI2CDevice>,
) -> io::Result<()> {
    if let Some(dev) = api.device_list().next() {
        if let Ok(wheel) = dev.open_device(api) {
            let mut buf: [u8; 32] = [0; 32];

            loop {
                // TODO: maybe put drawing on another thread and read (wheel data) + write (to dac via i2c) on a
                // different one.
                wheel.read(&mut buf).unwrap();
                let wheel_data: SteeringWheelData = buf.into();

                // TODO: could make DAC utils to keep supply voltage and make conversion instead of
                // manually calling the function.
                let throttle_voltage = {
                    if wheel_data.brake() < 1020 {
                        value_to_voltage(wheel_data.brake(), 1020, 1.0, 1.65)
                    } else if wheel_data.gas() > 0 {
                        value_to_voltage(wheel_data.gas(), 1020, 1.0, 1.65)
                    } else {
                        1.65
                    }
                };

                // gas_dac
                //     .write_dac_register(convert_value_to_dac(throttle_voltage, 4095.0, 3.3))
                //     .unwrap();

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
                        &format!("gas: {} | brake: {}", wheel_data.gas(), wheel_data.brake()),
                        4,
                        stdout,
                    );
                    print_middle_row(
                        &format!(
                            "throttle_value: {} (DAC: {}), steering: {} (DAC: {})",
                            throttle_voltage,
                            convert_value_to_dac(throttle_voltage, 4095.0, 3.3),
                            wheel_data.steering_angle,
                            convert_value_to_dac(wheel_data.steering_angle as f32, 4095.0, 255.0)
                        ),
                        5,
                        stdout,
                    );
                }
            }
        }
    }

    Ok(())
}
