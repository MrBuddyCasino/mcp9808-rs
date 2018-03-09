use reg::Register;

const REGISTER_PTR: u8 = 0b1000;

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

pub trait Resolution {
    fn new(reg_data: [u8; 2]) -> Self;
    fn get_register_ptr() -> u8;
    fn get_resolution(&self) -> Result<ResolutionVal, u8>;
    fn set_resolution(&self, p: ResolutionVal);
    fn get_precision_factor(&self) -> Result<f32, u8>;
}

impl Resolution for Register {
    fn new(reg_data: [u8; 2]) -> Self {
        Register::new(REGISTER_PTR as u8, reg_data)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_resolution(&self) -> Result<ResolutionVal, u8> {
        let val: u8 = 0;
        match val {
            0b00 => Ok(ResolutionVal::RES_0_5C),
            0b01 => Ok(ResolutionVal::RES_0_25C),
            0b10 => Ok(ResolutionVal::RES_0_125C),
            0b11 => Ok(ResolutionVal::RES_0_0625C),
            _ => Err(val)
        }
    }

    fn set_resolution(&self, _p: ResolutionVal) {
        unimplemented!()
    }

    fn get_precision_factor(&self) -> Result<f32, u8> {
        match self.get_resolution()? {
            ResolutionVal::RES_0_0625C => Ok(0.0625),
            ResolutionVal::RES_0_125C => Ok(0.125),
            ResolutionVal::RES_0_25C => Ok(0.25),
            ResolutionVal::RES_0_5C => Ok(0.5)
        }
    }
}


//struct Resolution(f32);
//
//const RES_0_5C: Resolution = Resolution(0.5);
//const RES_0_25C: Resolution = Resolution(0.25);
//const RES_0_125C: Resolution = Resolution(0.125);
//const RES_0_0625C: Resolution = Resolution(0.0625);