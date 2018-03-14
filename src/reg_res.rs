use reg::Register;
use core::fmt::Debug;

const REGISTER_PTR: u8 = 0b1000;
const REGISTER_SIZE: u8 = 1;

#[allow(non_camel_case_types)]
pub enum ResolutionVal {
    /// tCONV = 30 ms typical
    RES_0_5C = 0b00,
    /// tCONV = 65 ms typical
    RES_0_25C = 0b01,
    /// tCONV = 130 ms typical
    RES_0_125C = 0b10,
    /// power-up default, tCONV = 250 ms typical
    RES_0_0625C = 0b11,
}

pub trait Resolution: Debug + Copy + Clone {
    fn get_register_ptr() -> u8;
    fn get_resolution(&self) -> Result<ResolutionVal, u8>;
    fn set_resolution(&mut self, p: ResolutionVal);
}

pub fn new() -> impl Resolution {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl Resolution for Register {
    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_resolution(&self) -> Result<ResolutionVal, u8> {
        let val: u8 = self.get_msb();
        match val {
            0b00 => Ok(ResolutionVal::RES_0_5C),
            0b01 => Ok(ResolutionVal::RES_0_25C),
            0b10 => Ok(ResolutionVal::RES_0_125C),
            0b11 => Ok(ResolutionVal::RES_0_0625C),
            _ => Err(val)
        }
    }

    fn set_resolution(&mut self, p: ResolutionVal) {
        self.set_msb(p as u8);
    }
}
