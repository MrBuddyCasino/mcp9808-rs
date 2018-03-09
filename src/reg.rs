use bit_field::BitField;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    /// points to a specific register in the sensor
    pointer: u8,
    /// register contents
    data: u16,
}

impl Register {
    pub fn new(ptr: u8, buf: [u8; 2]) -> Self {
        Register { pointer: ptr, data: Register::make_word(buf) }
    }

    pub fn get_ptr(&self) -> u8 {
        self.pointer
    }

    pub fn get_data(&self) -> u16 {
        self.data
    }

    pub fn hibyte(&self) -> u8 {
        ((self.data >> 8) & 0xff) as u8
    }

    pub fn lobyte(&self) -> u8 {
        (self.data & 0xff) as u8
    }

    pub fn get_bit(&self, offset: usize) -> bool {
        self.data.get_bit(offset)
    }

    pub fn set_bit(&mut self, offset: usize, val: bool) {
        self.data.set_bit(offset, val);
    }

    pub fn make_word(buf: [u8; 2]) -> u16 {
        let (hi, lo) = (buf[0], buf[1]);
        ((hi as u16) << 8) + (lo as u16)
    }
}
