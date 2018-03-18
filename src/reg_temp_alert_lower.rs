use reg::Register;
use reg_temp_generic::WritableTempRegister;

// Temperature Alert Upper Boundary and Lower Boundary Limit registers
const REGISTER_PTR: u8 = 0b0011;
const REGISTER_SIZE: u8 = 2;

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

pub trait LowerTemperatureAlert: WritableTempRegister {}
impl LowerTemperatureAlert for Register {}