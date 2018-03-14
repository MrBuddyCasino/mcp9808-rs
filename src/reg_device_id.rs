use reg::Register;
use core::fmt::Debug;

const REGISTER_PTR: u8 = 0b0111;
const DEVICE_ID: u8 = 0x04;
const REGISTER_SIZE: u8 = 2;


pub trait DeviceId: Debug + Copy + Clone {
    fn get_register_ptr() -> u8;

    /// is the deviceId what it should be?
    fn is_valid_device(&self) -> bool;

    /// upper byte: DeviceId
    fn get_device_id(&self) -> u8;

    /// lower byte: Device Revision
    fn get_device_rev(&self) -> u8;
}

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl DeviceId for Register {
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