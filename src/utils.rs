/// This function is used to convert a desired voltage to a DAC value depending on supply
/// voltage.
pub fn voltage_to_dac_value(desired_voltage: f32, dac_max: usize, voltage_max: f32) -> usize {
    ((dac_max as f32 * desired_voltage) / voltage_max) as usize
}

/// This functions gets a value (from 0 to `value_max`) and converts to a voltage.
pub fn value_to_voltage(
    value: usize,
    value_max: usize,
    voltage_offset: f32,
    voltage_max: f32,
) -> f32 {
    (value as f32 * voltage_max) / value_max as f32 + voltage_offset
}
