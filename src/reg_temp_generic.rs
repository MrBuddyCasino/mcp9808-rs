extern crate cast;

#[cfg(feature = "with_floating_point")]
use cast::f32;
#[cfg(feature = "with_floating_point")]
use core::f32;

use cast::i16;
use prelude::Read;
use prelude::Write;
use reg::Register;
use reg_res::ResolutionVal;

const RANGE_LIMIT: i16 = 256;
const BIT_SIGN: u8 = 0x10;

/// internal generic trait for temperature registers
///
/// bit 15-13 Unimplemented: Read as ‘0’
/// bit 12 Sign: Sign bit (0 = TA ≥ 0°C, 1 = TA < 0°C)
/// bit 11-2 tUPPER/tLOWER/tCRIT: Temperature Boundary bits in two’s complement format.
/// 2-3 fractional part, 4-11 decimal part
/// bit 1-0 Unimplemented: Read as ‘0
pub trait ReadableTempRegister: Read {
    /// degree celsius as float
    #[cfg(feature = "with_floating_point")]
    fn get_celsius(&self, res: ResolutionVal) -> f32;

    /// avoids floats, but only works up to 0.125 resolution
    fn get_milli_celsius(&self, res: ResolutionVal) -> i32;

    /// raw register value
    fn get_raw_value(&self) -> u16;
}

impl ReadableTempRegister for Register {
    #[cfg(feature = "with_floating_point")]
    fn get_celsius(&self, res: ResolutionVal) -> f32 {
        let high = self.get_msb() & 0x1f; // clear flags
        let low: u8 = self.get_lsb().unwrap();

        let temp_dec = get_decimal_part(high, low);

        let mut ftemp = f32(temp_dec);
        ftemp += get_fractional_part_float(res, low);
        ftemp
    }

    fn get_milli_celsius(&self, res: ResolutionVal) -> i32 {
        if res == ResolutionVal::Deg_0_0625C {
            panic!("precision invalid for milli C°")
        }
        let high = self.get_msb() & 0x1f; // clear flags
        let low: u8 = self.get_lsb().unwrap();

        let mut temp_dec = get_decimal_part(high, low) as i32 * 1000;
        temp_dec += get_fractional_part_dec(res, low) as i32;
        temp_dec
    }

    fn get_raw_value(&self) -> u16 {
        self.as_u16()
    }
}

pub trait WritableTempRegister: ReadableTempRegister + Write {
    #[cfg(feature = "with_floating_point")]
    fn set_celsius(&mut self, val: f32);

    fn set_milli_celsius(&mut self, val: i32);
}

impl WritableTempRegister for Register {
    #[cfg(feature = "with_floating_point")]
    fn set_celsius(&mut self, val: f32) {
        if val >= f32(RANGE_LIMIT) || val <= -f32(RANGE_LIMIT) {
            panic!(
                "temperature {} exceeds valid range of +-{}",
                val, RANGE_LIMIT
            )
        }

        let temp_dec: u16 = val as u16;
        let mut high = (temp_dec / 16) as u8;
        let mut low = (temp_dec * 16) as u8;

        if val < 0.0 {
            high |= BIT_SIGN; // set sign bit
        }

        // fract() is std only
        let fract: f32 = val - (temp_dec as f32);
        let mut fract_bits: u8 =
            (fract / get_precision_factor_float(ResolutionVal::Deg_0_0625C)) as u8;
        fract_bits &= 0b1100; // mask bit 0+1
        low += fract_bits;

        self.set_msb(high);
        self.set_lsb(low);
    }

    fn set_milli_celsius(&mut self, val: i32) {
        if val.abs() >= 1000 * RANGE_LIMIT as i32 {
            panic!(
                "temperature {} exceeds valid range of +-{}",
                val, RANGE_LIMIT
            )
        }

        let temp_dec: i32 = val / 1000;
        let mut high = (temp_dec / 16) as u8;
        let mut low = (temp_dec * 16) as u8;

        if val < 0 {
            high |= BIT_SIGN; // set sign bit
        }

        // fract() is std only
        let fract: i32 = val - (temp_dec * 1000);
        let mut fract_bits: u8 =
            (fract / get_precision_factor_dec(ResolutionVal::Deg_0_0625C) as i32) as u8;
        fract_bits &= 0b1100; // mask bit 0+1
        low += fract_bits;

        self.set_msb(high);
        self.set_lsb(low);
    }
}

fn get_decimal_part(mut high: u8, low: u8) -> i16 {
    high &= 0x1f; // clear flags

    // sign bit set, < 0°C
    if high & BIT_SIGN == BIT_SIGN {
        high &= 0x0f; // clear sign bit
        256 - (i16(high) * 16 + i16(low) / 16)
    } else {
        i16(high) * 16 + i16(low) / 16
    }
}

fn get_fractional_part_dec(res: ResolutionVal, low: u8) -> u16 {
    let fract: u16 = (low & 0x000F).into(); // mask nibble
    (fract >> (3 - res as u16)) * get_precision_factor_dec(res)
}

#[cfg(feature = "with_floating_point")]
fn get_fractional_part_float(res: ResolutionVal, low: u8) -> f32 {
    let fract = low & 0x000F; // mask nibble
    f32(fract >> (3 - res as u8)) * get_precision_factor_float(res)
}

//fn set_fractional_part_float(res: ResolutionVal, val: f32) {}
//fn set_fractional_part_dec(res: ResolutionVal, val: i32) {}

#[cfg(feature = "with_floating_point")]
fn get_precision_factor_float(res: ResolutionVal) -> f32 {
    match res {
        ResolutionVal::Deg_0_0625C => 0.0625,
        ResolutionVal::Deg_0_125C => 0.125,
        ResolutionVal::Deg_0_25C => 0.25,
        ResolutionVal::Deg_0_5C => 0.5,
    }
}

fn get_precision_factor_dec(res: ResolutionVal) -> u16 {
    match res {
        ResolutionVal::Deg_0_0625C => 62,
        ResolutionVal::Deg_0_125C => 125,
        ResolutionVal::Deg_0_25C => 250,
        ResolutionVal::Deg_0_5C => 500,
    }
}

#[cfg(test)]
mod tests {
    /// prevent auto-format fuckup
    use super::*;
    use reg_res::ResolutionVal;

    #[test]
    fn read_sensor_value() {
        let msb: u8 = 0b00000001;
        let lsb: u8 = 0b10010100;
        let mut reg = Register::new(1, 2);
        reg.set_buf([msb, lsb]);

        let temp = reg.get_celsius(ResolutionVal::Deg_0_0625C);
        assert_eq!(temp, 25.25);

        let temp = reg.get_milli_celsius(ResolutionVal::Deg_0_125C);
        assert_eq!(temp, 25250);
    }

    #[test]
    fn set_celsius_integer() {
        let mut reg = Register::new(1, 2);

        // example bit pattern taken from data sheet, page 23
        reg.set_celsius(90.0);
        assert_eq!(0b00000101, reg.get_msb());
        assert_eq!(0b10100000, reg.get_lsb().unwrap());

        let temp = reg.get_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90.00);

        let temp = reg.get_milli_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90000);
    }

    #[test]
    fn set_celsius_fractional() {
        let mut reg = Register::new(1, 2);

        // example bit pattern taken from data sheet, page 23
        reg.set_celsius(90.75);
        assert_eq!(0b00000101, reg.get_msb());
        assert_eq!(0b10101100, reg.get_lsb().unwrap());

        let temp = reg.get_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90.75);

        let temp = reg.get_milli_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90750);

        reg.set_celsius(90.25);
        assert_eq!(0b00000101, reg.get_msb());
        assert_eq!(0b10100100, reg.get_lsb().unwrap());
    }

    #[test]
    fn set_milli_celsius() {
        let mut reg = Register::new(1, 2);

        // example bit pattern taken from data sheet, page 23
        reg.set_milli_celsius(90000);
        assert_eq!(0b00000101, reg.get_msb());
        assert_eq!(0b10100000, reg.get_lsb().unwrap());

        let temp = reg.get_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90.00);

        let temp = reg.get_milli_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90000);
    }

    #[test]
    fn set_milli_celsius_fractional() {
        let mut reg = Register::new(1, 2);

        // example bit pattern taken from data sheet, page 23
        reg.set_milli_celsius(90250);
        assert_eq!(0b00000101, reg.get_msb());
        assert_eq!(0b10100100, reg.get_lsb().unwrap());

        let temp = reg.get_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90.25);

        let temp = reg.get_milli_celsius(ResolutionVal::Deg_0_25C);
        assert_eq!(temp, 90250);
    }
}
