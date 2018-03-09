//! Traits for

use hal::blocking::i2c;

/// Trait for writing data to SSD1306
pub trait Write {
    /// Error type
    type WriteError;

    /// Write a command to SSD1306
    fn write_register(&mut self, addr: u8, reg_ptr: u8, reg_data: [u8; 2]) -> Result<(), Self::WriteError>;
}

impl<I2C> Write for I2C
    where
        I2C: i2c::Write,
{
    type WriteError = I2C::Error;

    fn write_register(&mut self, addr: u8, reg_ptr: u8, reg_data: [u8; 2]) -> Result<(), Self::WriteError> {
        let mut buf = [reg_ptr, reg_data[0], reg_data[1]];
        self.write(addr, &mut buf)
    }
}

pub trait Read {
    /// Error type
    type ReadError;

    fn read_register(&mut self, addr: u8, reg_ptr: u8) -> Result<[u8; 2], Self::ReadError>;
}

impl<I2C> Read for I2C
    where
        I2C: i2c::WriteRead,
{
    type ReadError = I2C::Error;

    fn read_register(&mut self, addr: u8, reg_ptr: u8) -> Result<[u8; 2], Self::ReadError> {
        let mut buf: [u8; 2] = [0, 0];
        self.write_read(addr, &[reg_ptr], &mut buf)?;
        Ok(buf)
    }
}