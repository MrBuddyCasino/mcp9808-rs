extern crate cast;

use cast::f32;
use reg::Register;

const REGISTER_PTR: u8 = 0b0101;

pub trait Temperature {
    fn new(reg_data: [u8; 2]) -> Self;
    fn get_register_ptr() -> u8;
    fn temperature(&self) -> f32;
}

impl Temperature for Register {
    fn new(reg_data: [u8; 2]) -> Self {
        Register::new(REGISTER_PTR as u8, reg_data)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn temperature(&self) -> f32 {
        let buf = [self.hibyte(), self.lobyte()];
        convert_temperature(buf)
    }
}

fn convert_temperature(temp_raw: [u8; 2]) -> f32 {
    let (mut high, low) = (temp_raw[0], temp_raw[1]);
    high = high & 0x1f; // clear flags

    // < 0°C
    let mut ftemp: f32;
    if high & 0x10 == 0x10 {
        high = high & 0x0f;
        ftemp = (f32(high) * 16.0 + f32(low) / 16.0) - 256 as f32;
    } else {
        ftemp = (f32(high) * 16.0 + f32(low) / 16.0) as f32;
    }

    ftemp += 0.0625f32 * f32(low & 0x000F);

    ftemp
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