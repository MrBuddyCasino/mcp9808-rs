use reg::Register;

const REGISTER_PTR: u8 = 0b0111;
const DEVICE_ID: u8 = 0x04;
const REGISTER_SIZE: u8 = 2;

/// upper byte: DeviceId, lower byte: Device Revision
pub trait DeviceId {
    fn new(buf: &[u8]) -> Result<Self, u8> where Self: Sized;
    fn get_register_ptr() -> u8;
    fn is_valid_device(&self) -> bool;
    fn get_device_id(&self) -> u8;
    fn get_device_rev(&self) -> u8;
}

impl DeviceId for Register {
    fn new(buf: &[u8]) -> Result<Self, u8> {
        Register::new(REGISTER_PTR, &buf, REGISTER_SIZE)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    /// are we talking to the right device?
    fn is_valid_device(&self) -> bool {
        self.get_device_id() == DEVICE_ID
    }

    /// get device id
    fn get_device_id(&self) -> u8 {
        self.get_msb()
    }

    /// get device revision
    fn get_device_rev(&self) -> u8 {
        self.get_lsb().unwrap()
    }
}