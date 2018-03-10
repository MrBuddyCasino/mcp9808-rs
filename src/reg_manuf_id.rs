use reg::Register;

const REGISTER_PTR: u8 = 0b0110;
const MANUFACTURER_ID: u16 = 0x0054;
const REGISTER_SIZE: u8 = 2;

pub trait ManufacturerId {
    fn new(buf: &[u8]) -> Result<Self, u8> where Self: Sized;
    fn get_register_ptr() -> u8;
    fn get_manufacturer_id(&self) -> u16;
    fn is_valid_manufacturer(&self) -> bool;
}

impl ManufacturerId for Register {
    /// construct from i2c buffer
    fn new(buf: &[u8]) -> Result<Self, u8> {
        Register::new(REGISTER_PTR, &buf, REGISTER_SIZE)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_manufacturer_id(&self) -> u16 {
        // buffer is guaranteed to be 2 bytes here
        self.as_u16()
    }

    /// are we talking to the right device?
    fn is_valid_manufacturer(&self) -> bool {
        self.get_manufacturer_id() == MANUFACTURER_ID
    }
}