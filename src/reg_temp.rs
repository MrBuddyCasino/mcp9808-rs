extern crate cast;

use cast::f32;
use reg::Register;

const REGISTER_PTR: u8 = 0b0101;
const REGISTER_SIZE: u8 = 2;

pub trait Temperature {
    fn new(buf: &[u8]) -> Result<Self, u8> where Self: Sized;
    fn get_register_ptr() -> u8;
    fn get_temperature(&self) -> f32;
}

impl Temperature for Register {
    fn new(buf: &[u8]) -> Result<Self, u8> {
        Register::new(REGISTER_PTR, &buf, REGISTER_SIZE)
    }

    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_temperature(&self) -> f32 {
        convert_temperature(self.get_msb(), self.get_lsb().unwrap())
    }
}

fn convert_temperature(high_raw: u8, low: u8) -> f32 {
    let mut high = high_raw & 0x1f; // clear flags

    // < 0°C
    let mut ftemp: f32;
    if high & 0x10 == 0x10 {
        high = high & 0x0f;
        ftemp = (f32(high) * 16.0 + f32(low) / 16.0) as f32 - 256.0;
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