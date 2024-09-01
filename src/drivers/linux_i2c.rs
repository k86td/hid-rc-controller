use embedded_hal::i2c::{Error, ErrorKind, ErrorType, I2c};
use linux_embedded_hal::i2cdev::{
    core::I2CDevice,
    linux::{LinuxI2CDevice, LinuxI2CError},
};

pub struct ExtendedLinuxI2CDevice {
    dev: LinuxI2CDevice,
}

#[derive(Debug)]
pub struct ExtendedLinuxI2CError {
    inner: LinuxI2CError,
}

impl Error for ExtendedLinuxI2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match &self.inner {
            // TODO: not sure the errors here are even doing anything since we can't identify them
            LinuxI2CError::Errno(_) => ErrorKind::Other,
            LinuxI2CError::Io(_) => ErrorKind::Other,
        }
    }
}

impl ErrorType for ExtendedLinuxI2CDevice {
    type Error = ExtendedLinuxI2CError;
}

impl I2c for ExtendedLinuxI2CDevice {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let _ = address;

        for operation in operations {
            let err = match operation {
                embedded_hal::i2c::Operation::Read(buf) => self.dev.read(buf),
                embedded_hal::i2c::Operation::Write(buf) => self.dev.write(buf),
            };

            // TODO: figure out how to implement the `?` operator.
            if let Err(e) = err {
                return Err(ExtendedLinuxI2CError { inner: e });
            }
        }

        Ok(())
    }
}
