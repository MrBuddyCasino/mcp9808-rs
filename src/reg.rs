use bit_field::BitArray;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    /// points to a specific register in the sensor
    ptr: u8,
    /// register contents, either 1 or 2 bytes
    buf: [u8; 2],
    len: u8,
}

impl Register {
    pub fn new(ptr: u8, buffer: &[u8], len: u8) -> Result<Self, u8> {
        if buffer.len() != len as usize {
            return Err(buffer.len() as u8);
        }

        // 2nd byte is optional
        let lsb = buffer.get(1).cloned().unwrap_or(0);
        let buf = [buffer[0], lsb];
        Ok(Register { ptr, buf, len })
    }

    pub fn get_buf(&self) -> &[u8] {
        &self.buf[0..self.len as usize]
    }

    pub fn get_ptr(&self) -> u8 {
        self.ptr
    }

    /// lower byte, bits 0-7, availability depends on register type
    pub fn get_lsb(&self) -> Option<u8> {
        self.buf.get(1).cloned()
    }

    /// upper byte, bits 8-15, always available
    pub fn get_msb(&self) -> u8 {
        self.buf[0]
    }

    pub fn set_lobyte(&mut self, val: u8) {
        self.buf[0] = val;
    }

    pub fn get_bit(&self, offset: usize) -> bool {
        let bit = if offset > 7 { 15 - offset } else { offset };
        self.buf.get_bit(bit)
    }

    /// datasheet numbers bits from lsb-0, we store them as msb
    pub fn set_bit(&mut self, offset: usize, val: bool) {
        let bit = if offset > 7 { 15 - offset } else { offset };
        self.buf.set_bit(bit, val);
    }

    pub fn as_u16(&self) -> Option<u16> {
        let (lo, hi) = (self.get_lsb(), self.get_msb());
        if lo.is_none() { () }
        Some(((hi as u16) << 8) + (lo.unwrap() as u16))
    }
}
