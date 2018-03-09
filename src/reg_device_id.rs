use reg::Register;

const REGISTER_PTR: u8 = 0b0111;
const DEVICE_ID: u8 = 0x04;

/// upper byte: DeviceId, lower byte: Device Revision
pub trait DeviceId {
    fn new(reg_data: [u8; 2]) -> Self;
    fn get_register_ptr() -> u8;
    fn is_valid_device(&self) -> bool;
    fn get_device_id(&self) -> u8;
    fn get_device_rev(&self) -> u8;
}

impl DeviceId for Register {
    fn new(reg_data: [u8; 2]) -> Self {
        Register::new(REGISTER_PTR as u8, reg_data)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    /// are we talking to the right device?
    fn is_valid_device(&self) -> bool {
        self.get_device_id() == DEVICE_ID
    }

    fn get_device_id(&self) -> u8 {
        self.hibyte()
    }

    fn get_device_rev(&self) -> u8 {
        self.lobyte()
    }
}