#![deny(warnings)]
#![no_std]

extern crate cast;
extern crate embedded_hal as hal;
extern crate bit_field;

pub mod prelude;
pub mod reg;
pub mod reg_conf;
pub mod reg_device_id;
pub mod reg_manuf_id;
pub mod reg_res;
pub mod reg_temp;


use hal::blocking::i2c;
use reg_temp::Temperature;
use reg_conf::Configuration;
use reg_manuf_id::ManufacturerId;
use reg_device_id::DeviceId;
use reg::Register;
use prelude::Read;
use prelude::Write;

/// I2C address
#[derive(Clone, Copy)]
pub enum Address {
    Default = 0b0011000
}


/// MCP9808 Driver
pub struct MCP9808<I2C> {
    addr: u8,
    i2c: I2C,
}


impl<I2C, E> MCP9808<I2C>
    where I2C: i2c::Write<Error=E> + i2c::WriteRead<Error=E>

{
    /// Creates a new driver from an I2C peripheral.
    pub fn new(i2c: I2C) -> Self {
        MCP9808 {
            addr: Address::Default as u8,
            i2c,
        }
    }

    /// Release resources
    pub fn free(self) -> I2C {
        self.i2c
    }

    pub fn read_manufacturer_id(&mut self) -> Result<Register, E> {
        let ptr = <Register as ManufacturerId>::get_register_ptr();
        let reg = self.read_register(ptr)?;
        Ok(reg)
    }

    pub fn read_device_id(&mut self) -> Result<Register, E> {
        let ptr = <Register as DeviceId>::get_register_ptr();
        let reg = self.read_register(ptr)?;
        Ok(reg)
    }

    /// send register content to sensor
    fn write_register(&mut self, reg: Register) -> Result<(), E>
    {
        &self.i2c.write_register(self.addr, reg.get_ptr(), [reg.hibyte(), reg.lobyte()])?;
        Ok(())
    }

    /// read register from sensor
    fn read_register(&mut self, ptr: u8) -> Result<Register, E>
    {
        let buf = &self.i2c.read_register(self.addr, ptr)?;
        let reg = Register::new(ptr, *buf);
        Ok(reg)
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let ptr = <Register as Temperature>::get_register_ptr();
        let reg = self.read_register(ptr)?;
        return Ok(reg.temperature());
    }

    pub fn read_configuration(&mut self) -> Result<Register, E> {
        let ptr = <Register as Configuration>::get_register_ptr();
        let reg = self.read_register(ptr)?;
        return Ok(reg);
    }

    pub fn write_configuration(&mut self, reg: Register) -> Result<(), E> {
        self.write_register(reg)?;
        return Ok(());
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}