use reg::Register;
use reg_temp_generic::WritableTempRegister;

// Critical Temperature Limit register
const REGISTER_PTR: u8 = 0b0100;
const REGISTER_SIZE: u8 = 2;

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

pub trait CriticalTemperatureAlert: WritableTempRegister {}

impl CriticalTemperatureAlert for Register {}