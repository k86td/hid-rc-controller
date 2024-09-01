extern crate hid_rc_controller;

// TODO: move this back to main?
#[cfg(test)]
mod tests {
    use hid_rc_controller::utils::{value_to_voltage, voltage_to_dac_value};

    #[test]
    fn get_right_value_voltage_to_dac_value() {
        assert_eq!(voltage_to_dac_value(2.65, 4095, 3.3), 3288);
        assert_eq!(voltage_to_dac_value(0.0, 4095, 3.3), 0);
    }

    #[test]
    fn get_value_to_voltage() {
        assert_eq!(value_to_voltage(0, 1020, 1.0, 1.65), 1.0);
        assert_eq!(value_to_voltage(1020, 1020, 1.0, 1.65), 2.65);
    }
}
