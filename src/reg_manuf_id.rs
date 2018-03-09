use reg::Register;

const REGISTER_PTR: u8 = 0b0110;
const MANUFACTURER_ID: u16 = 0x0054;

pub trait ManufacturerId {
    fn new(reg_data: [u8; 2]) -> Self;
    fn get_register_ptr() -> u8;
    fn get_manufacturer_id(&self) -> u16;
    fn is_valid_manufacturer(&self) -> bool;
}

impl ManufacturerId for Register {
    fn new(reg_data: [u8; 2]) -> Self {
        Register::new(REGISTER_PTR as u8, reg_data)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_manufacturer_id(&self) -> u16 {
        self.get_data()
    }

    /// are we talking to the right device?
    fn is_valid_manufacturer(&self) -> bool {
        self.get_manufacturer_id() == MANUFACTURER_ID
    }
}