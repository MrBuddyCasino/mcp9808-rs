extern crate cast;

use cast::f32;
use reg::Register;
use reg_res::ResolutionVal;

const REGISTER_PTR: u8 = 0b0101;
const REGISTER_SIZE: u8 = 2;

pub trait Temperature {
    fn new(buf: &[u8]) -> Result<Self, u8> where Self: Sized;
    fn get_register_ptr() -> u8;
    fn is_temp_critical(&self) -> bool;
    fn get_temperature(&self, res: ResolutionVal) -> f32;
}

impl Temperature for Register {
    fn new(buf: &[u8]) -> Result<Self, u8> {
        Register::new(REGISTER_PTR, &buf, REGISTER_SIZE)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn is_temp_critical(&self) -> bool {
        self.get_bit(15)
    }

    fn get_temperature(&self, res: ResolutionVal) -> f32 {
        let mut high = self.get_msb() & 0x1f; // clear flags
        let low: u8 = self.get_lsb().unwrap();

        // < 0°C
        let mut ftemp: f32;
        if high & 0x10 == 0x10 {
            high = high & 0x0f;
            ftemp = (f32(high) * 16.0 + f32(low) / 16.0) as f32 - 256.0;
        } else {
            ftemp = (f32(high) * 16.0 + f32(low) / 16.0) as f32;
        }

        ftemp += get_precision_factor(res) * f32(low & 0x000F);

        ftemp
    }
}

fn get_precision_factor(res: ResolutionVal) -> f32 {
    match res {
        ResolutionVal::RES_0_0625C => 0.0625,
        ResolutionVal::RES_0_125C => 0.125,
        ResolutionVal::RES_0_25C => 0.25,
        ResolutionVal::RES_0_5C => 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn temp_crit() {
        let msb: u8 = 0b10000000;
        let lsb: u8 = 0b00000000;
        let mut reg: Register = Temperature::new(&[msb, lsb]).unwrap();
        assert_eq!(reg.is_temp_critical(), true);

        reg.set_bit(15, false);
        assert_eq!(reg.is_temp_critical(), false);
    }

        #[test]
    fn temp_conversion() {
        let msb: u8 = 0b00000001;
        let lsb: u8 = 0b10010100;
        let reg: Register = Temperature::new(&[msb, lsb]).unwrap();

        assert_eq!(reg.is_temp_critical(), false);

        let temp = reg.get_temperature(ResolutionVal::RES_0_0625C);
        assert_eq!(temp, 25.25);
    }
}


//pub struct MCP9808_REG__TEMP_AMB {
//    value: u16
//    /*
//    Decimal 0, 4,
//    Integer 4, 4,
//    Sign 8, 1,
//    VsTLow 9, 1,
//    VsTHigh 10, 1,
//    VsTCrit 11, 1
//    */
//}