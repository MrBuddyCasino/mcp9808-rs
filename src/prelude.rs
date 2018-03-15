use hal::blocking::i2c;
use core::fmt::Debug;
use reg::Register;

/// trait for a register that can be read from an i2c device
pub trait Read: Debug + Copy + Clone {
    fn read_from_device<I2C, E>(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), E>
        where I2C: i2c::WriteRead<Error=E>;
}

impl Read for Register {
    fn read_from_device<I2C, E>(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), E>
        where I2C: i2c::WriteRead<Error=E> {
        let mut buf = [0u8; 2];
        i2c.write_read(addr, &[self.get_ptr()], &mut buf[0..self.get_len() as usize])?;
        self.set_buf(buf);
        Ok(())
    }
}


/// trait for a register that can be written to an i2c device
pub trait Write: Read {
    fn write_to_device<I2C, E>(&self, i2c: &mut I2C, addr: u8) -> Result<(), E>
        where I2C: i2c::Write<Error=E>;
}

impl Write for Register {
    fn write_to_device<I2C, E>(&self, i2c: &mut I2C, addr: u8) -> Result<(), E>
        where I2C: i2c::Write<Error=E> {
        // reg ptr + 1 or 2 bytes
        let mut buf = [self.get_ptr(); 3];
        for (i, item) in self.get_buf().iter().enumerate() {
            buf[i + 1] = *item;
        }
        i2c.write(addr, &buf[0..self.get_len() as usize])
    }
}