use bit_field::BitField;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    /// points to a specific register in the sensor
    ptr: u8,
    /// register contents, either 1 or 2 bytes
    buf: [u8; 2],
    /// actual register size in bytes, either 1 or 2
    len: u8,
}


impl Register {
    pub fn new(ptr: u8, len: u8) -> Self {
        if ptr == 0 || ptr > 0b1000u8 {
            panic!("invalid pointer value: reserved")
        }

        if len > 2 {
            panic!("length > 2")
        }

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

    pub fn get_len(&self) -> u8 {
        self.len
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
        let mut reg: Register = Register::new(1, 2);

        assert_eq!(reg.as_u16(), 0);

        assert_eq!(reg.get_bit(15), false);
        reg.set_bit(15, true);
        assert_eq!(reg.get_bit(15), true);

        assert_eq!(reg.get_bit(0), false);
        reg.set_bit(0, true);
        assert_eq!(reg.get_bit(0), true);
    }
}