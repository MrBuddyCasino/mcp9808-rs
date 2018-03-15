use reg::Register;
use prelude::Read;

const REGISTER_PTR: u8 = 0b0110;
const MANUFACTURER_ID: u16 = 0x0054;
const REGISTER_SIZE: u8 = 2;

pub trait ManufacturerId: Read {
    fn get_manufacturer_id(&self) -> u16;
    fn is_valid_manufacturer(&self) -> bool;
}

/// construct from i2c buffer
pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl ManufacturerId for Register {
    fn get_manufacturer_id(&self) -> u16 {
        // buffer is guaranteed to be 2 bytes here
        self.as_u16()
    }

    /// are we talking to the right device?
    fn is_valid_manufacturer(&self) -> bool {
        self.get_manufacturer_id() == MANUFACTURER_ID
    }
}