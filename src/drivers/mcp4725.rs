// this is a driver for MCP4725 chip.
// this is heavily inspired by:
// https://github.com/vgasparyan/mcp4725-rs/blob/master/src/lib.rs

use embedded_hal::i2c::{ErrorType, I2c};

pub struct MCP4725<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C: I2c> MCP4725<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
}

impl<I2C: I2c> MCP4725<I2C> {
    pub fn write_dac_register_fast(
        &mut self,
        input_code: u16,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        let b1 = (input_code >> 8) as u8;
        let b2 = (input_code & 0xFF) as u8;

        self.i2c.write(self.address, &[b1, b2])?;

        Ok(())
    }

    pub fn write_dac_register(&mut self, input_code: u16) -> Result<(), <I2C as ErrorType>::Error> {
        let b1 = (input_code >> 4) as u8;
        let b2 = (input_code << 4) as u8;

        self.i2c.write(self.address, &[0x40, b1, b2])?;

        Ok(())
    }
}
