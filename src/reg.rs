use bit_field::BitField;
use hal::blocking::i2c;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    /// points to a specific register in the sensor
    ptr: u8,
    /// register contents, either 1 or 2 bytes
    buf: [u8; 2],
    /// actual register size in bytes, either 1 or 2
    len: u8,
}

/// trait for a register that can be read from an i2c device
pub trait Read<I2C, E> {
    /// Error type
    type ReadError;

    fn read_from_device(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), E>;
}


impl<I2C, E> Read<I2C, E> for Register
    where
        I2C: i2c::WriteRead<Error=E>, {
    type ReadError = I2C::Error;

    fn read_from_device(&mut self, i2c: &mut I2C, addr: u8) -> Result<(), E> {
        i2c.write_read(addr, &[self.ptr], &mut self.buf[0..self.len as usize])?;
        Ok(())
    }
}

/// trait for a register that can be written to an i2c device
pub trait Write<I2C> {
    /// Error type
    type WriteError;

    fn write_to_device(&self, i2c: &mut I2C, addr: u8) -> Result<(), Self::WriteError>;
}

impl<I2C> Write<I2C> for Register
    where I2C: i2c::Write {
    type WriteError = I2C::Error;

    fn write_to_device(&self, i2c: &mut I2C, addr: u8) -> Result<(), Self::WriteError> {
        // reg ptr + 1 or 2 bytes
        let mut buf = [self.get_ptr(); 3];
        for (i, item) in self.get_buf().iter().enumerate() {
            buf[i + 1] = *item;
        }
        i2c.write(addr, &buf[0..self.len as usize])
    }
}

impl Register {
    pub fn new(ptr: u8, len: u8) -> Self {
        let buf = [0u8, 0];
        Register { ptr, buf, len }
    }

    pub fn get_buf(&self) -> &[u8] {
        &self.buf[0..self.len as usize]
    }

    pub fn set_buf(&mut self, val: [u8; 2]) {
        self.buf = val;
    }

    pub fn get_ptr(&self) -> u8 {
        self.ptr
    }

    /// lower byte, bits 0-7, availability depends on register type
    pub fn get_lsb(&self) -> Option<u8> {
        if self.len < 2 {
            return None;
        }
        Some(self.buf[1])
    }

    pub fn set_lsb(&mut self, val: u8) {
        self.buf[1] = val;
    }

    /// upper byte, bits 8-15, always available
    pub fn get_msb(&self) -> u8 {
        self.buf[0]
    }

    pub fn set_msb(&mut self, val: u8) {
        self.buf[0] = val;
    }

    pub fn get_bit(&self, offset: usize) -> bool {
        if self.len == 1 {
            return self.get_msb().get_bit(offset);
        }

        if offset > 7 {
            return self.get_msb().get_bit(offset - 8);
        } else {
            return self.get_lsb().unwrap().get_bit(offset);
        }
    }

    /// datasheet numbers bits from lsb-0, we store them as msb
    pub fn set_bit(&mut self, offset: usize, val: bool) {
        if offset + 1 > self.len as usize * 8 {
            panic!("out of bounds access")
        }

        if self.len == 1 {
            self.buf[0].set_bit(offset, val);
        }

        if offset > 7 {
            self.buf[0].set_bit(offset - 8, val);
        } else {
            self.buf[1].set_bit(offset, val);
        }
    }

    pub fn as_u16(&self) -> u16 {
        let (lo, hi) = (self.get_lsb(), self.get_msb());
        if lo.is_none() { return self.get_msb() as u16; }
        ((hi as u16) << 8) + (lo.unwrap() as u16)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitfield_manipulation() {
        let mut reg: Register = Register::new(0, 2);

        assert_eq!(reg.as_u16(), 0);

        assert_eq!(reg.get_bit(15), false);
        reg.set_bit(15, true);
        assert_eq!(reg.get_bit(15), true);

        assert_eq!(reg.get_bit(0), false);
        reg.set_bit(0, true);
        assert_eq!(reg.get_bit(0), true);
    }
}