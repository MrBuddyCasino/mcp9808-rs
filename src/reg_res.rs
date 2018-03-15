use reg::Register;
use prelude::Write;

const REGISTER_PTR: u8 = 0b1000;
const REGISTER_SIZE: u8 = 1;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ResolutionVal {
    /// tCONV = 30 ms typical
    Deg_0_5C = 0b00,
    /// tCONV = 65 ms typical
    Deg_0_25C = 0b01,
    /// tCONV = 130 ms typical
    Deg_0_125C = 0b10,
    /// power-up default, tCONV = 250 ms typical
    Deg_0_0625C = 0b11,
}

pub trait Resolution: Write {
    fn get_resolution(&self) -> ResolutionVal;
    fn set_resolution(&mut self, p: ResolutionVal);
}

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl Resolution for Register {
    fn get_resolution(&self) -> ResolutionVal {
        let val: u8 = self.get_msb();
        match val {
            0b00 => ResolutionVal::Deg_0_5C,
            0b01 => ResolutionVal::Deg_0_25C,
            0b10 => ResolutionVal::Deg_0_125C,
            0b11 => ResolutionVal::Deg_0_0625C,
            _ => panic!("invalid resolution")
        }
    }

    fn set_resolution(&mut self, p: ResolutionVal) {
        self.set_msb(p as u8);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution() {
        let mut reg: Register = new();
        assert_eq!(reg.get_resolution(), ResolutionVal::Deg_0_5C);

        reg.set_resolution(ResolutionVal::Deg_0_25C);
        assert_eq!(reg.get_resolution(), ResolutionVal::Deg_0_25C);

        reg.set_resolution(ResolutionVal::Deg_0_125C);
        assert_eq!(reg.get_resolution(), ResolutionVal::Deg_0_125C);

        reg.set_resolution(ResolutionVal::Deg_0_0625C);
        assert_eq!(reg.get_resolution(), ResolutionVal::Deg_0_0625C);

        reg.set_resolution(ResolutionVal::Deg_0_5C);
        assert_eq!(reg.get_resolution(), ResolutionVal::Deg_0_5C);
    }
}