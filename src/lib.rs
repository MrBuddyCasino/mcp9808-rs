#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate cast;
extern crate embedded_hal as hal;

use hal::blocking::i2c;
use prelude::Read;
use prelude::Write;
use reg::Register;
use reg_conf::Configuration;
use reg_device_id::DeviceId;
use reg_manuf_id::ManufacturerId;
use reg_temp::Temperature;

pub mod prelude;
pub mod reg;
pub mod reg_conf;
pub mod reg_device_id;
pub mod reg_manuf_id;
pub mod reg_res;
pub mod reg_temp;


/// I2C address
#[derive(Clone, Copy)]
pub enum Address {
    Default = 0b0011000
}

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
    RegisterSizeMismatch(u8),
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

    fn register_from_ic2<O: FnOnce(&[u8]) -> Result<Register, u8>>(&mut self, reg_ptr: u8, maker: O) -> Result<Register, Error<E>> {
        let buf = &self.i2c.read_register(self.addr, reg_ptr).map_err(Error::I2c)?;
        let reg = maker(buf).map_err(|e: u8| Error::RegisterSizeMismatch(e))?;
        Ok(reg)
    }

    /// send register content to sensor
    fn write_register(&mut self, reg: Register) -> Result<(), Error<E>>
    {
        &self.i2c.write_register(self.addr, reg.get_ptr(), reg.get_buf()).map_err(Error::I2c)?;
        Ok(())
    }

    pub fn read_manufacturer_id(&mut self) -> Result<Register, Error<E>> {
        let ptr = <Register as ManufacturerId>::get_register_ptr();
        self.register_from_ic2(ptr, |buf: &[u8]| ManufacturerId::new(buf))
    }

    pub fn read_device_id(&mut self) -> Result<Register, Error<E>> {
        let ptr = <Register as DeviceId>::get_register_ptr();
        self.register_from_ic2(ptr, |buf: &[u8]| DeviceId::new(buf))
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature(&mut self) -> Result<Register, Error<E>> {
        let ptr = <Register as Temperature>::get_register_ptr();
        self.register_from_ic2(ptr, |buf: &[u8]| Temperature::new(buf))
    }

    pub fn read_configuration(&mut self) -> Result<Register, Error<E>> {
        let ptr = <Register as Configuration>::get_register_ptr();
        self.register_from_ic2(ptr, |buf: &[u8]| Configuration::new(buf))
    }

    pub fn write_configuration(&mut self, reg: Register) -> Result<(), Error<E>> {
        self.write_register(reg)?;
        return Ok(());
    }
}
