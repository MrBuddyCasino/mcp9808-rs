use crate::error::Error;
use crate::reg::Register;
use core::fmt::Debug;
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// trait for a register that can be read from an i2c device
pub trait Read: Debug + Copy + Clone {
    fn read_from_device<I2C>(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>;
}

impl Read for Register {
    fn read_from_device<I2C>(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        let mut buf = [0u8; 2];
        i2c.write_read(
            addr,
            &[self.get_ptr()],
            &mut buf[0..self.get_len() as usize],
        )?;
        self.set_buf(buf);
        Ok(())
    }
}

/// trait for a register that can be written to an i2c device
pub trait Write: Read {
    fn write_to_device<I2C>(&self, i2c: &mut I2C, addr: u8) -> Result<(), Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>;
}

impl Write for Register {
    fn write_to_device<I2C>(&self, i2c: &mut I2C, addr: u8) -> Result<(), Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        // reg ptr + 1 or 2 bytes
        let mut buf = [self.get_ptr(); 3];
        for (i, item) in self.get_buf().iter().enumerate() {
            buf[i + 1] = *item;
        }
        Ok(i2c.write(addr, &buf[0..self.get_len() as usize])?)
    }
}
