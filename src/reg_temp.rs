extern crate cast;

use cast::u16;
use cast::f32;
use reg::Register;
use reg_res::ResolutionVal;
use prelude::*;

const REGISTER_PTR: u8 = 0b0101;
const REGISTER_SIZE: u8 = 2;

const BIT_ALERT_CRITICAL: usize = 15;
const BIT_ALERT_UPPER: usize = 14;
const BIT_ALERT_LOWER: usize = 13;

pub trait Temperature: Read {
    fn is_alert_critical(&self) -> bool;
    fn is_alert_upper(&self) -> bool;
    fn is_alert_lower(&self) -> bool;
    fn get_temperature(&self, res: ResolutionVal) -> f32;
    fn get_raw_value(&self) -> u16;
}

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl Temperature for Register {
    fn is_alert_critical(&self) -> bool {
        self.get_bit(BIT_ALERT_CRITICAL)
    }
    fn is_alert_upper(&self) -> bool {
        self.get_bit(BIT_ALERT_UPPER)
    }
    fn is_alert_lower(&self) -> bool {
        self.get_bit(BIT_ALERT_LOWER)
    }

    fn get_temperature(&self, res: ResolutionVal) -> f32 {
        let mut high = self.get_msb() & 0x1f; // clear flags
        let low: u8 = self.get_lsb().unwrap();

        let temp: u16;

        // sign bit set, < 0°C
        if high & 0x10 == 0x10 {
            high = high & 0x0f; // clear sign bit
            temp = 256 - (u16(high) * 16 + u16(low) / 16);
        } else {
            temp = u16(high) * 16 + u16(low) / 16;
        }

        let mut ftemp = f32(temp);
        let fract = low & 0x000F; // mask nibble
        ftemp += f32(fract >> (3 - res as u8)) * get_precision_factor(res);

        ftemp
    }

    fn get_raw_value(&self) -> u16 {
        *&self.as_u16()
    }
}

fn get_precision_factor(res: ResolutionVal) -> f32 {
    match res {
        ResolutionVal::Deg_0_0625C => 0.0625,
        ResolutionVal::Deg_0_125C => 0.125,
        ResolutionVal::Deg_0_25C => 0.25,
        ResolutionVal::Deg_0_5C => 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn alert_critical() {
        let mut reg = new();

        assert_eq!(reg.is_alert_critical(), false);
        reg.set_bit(BIT_ALERT_CRITICAL, true);
        assert_eq!(reg.is_alert_critical(), true);
    }

    #[test]
    fn alert_upper() {
        let mut reg = new();

        assert_eq!(reg.is_alert_upper(), false);
        reg.set_bit(BIT_ALERT_UPPER, true);
        assert_eq!(reg.is_alert_upper(), true);
    }

    #[test]
    fn alert_lower() {
        let mut reg = new();

        assert_eq!(reg.is_alert_lower(), false);
        reg.set_bit(BIT_ALERT_LOWER, true);
        assert_eq!(reg.is_alert_lower(), true);
    }

    #[test]
    fn temp_conversion() {
        let msb: u8 = 0b00000001;
        let lsb: u8 = 0b10010100;
        let mut reg = new();
        reg.set_buf([msb, lsb]);

        let temp = reg.get_temperature(ResolutionVal::Deg_0_0625C);
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