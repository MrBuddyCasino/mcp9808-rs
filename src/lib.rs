#![feature(conservative_impl_trait, universal_impl_trait)]
//#![feature(trait_alias)]

#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate cast;
extern crate embedded_hal as hal;

use hal::blocking::i2c;
use reg_conf::Configuration;
use reg_device_id::DeviceId;
use reg_manuf_id::ManufacturerId;
use reg_temp::Temperature;

pub mod reg;
mod prelude;
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

    /// release resources
    pub fn free(self) -> I2C {
        self.i2c
    }

    fn read_register<T>(&mut self, mut reg: T) -> Result<T, Error<E>>
        where T: prelude::Read,
              I2C: i2c::WriteRead {
        reg.read_from_device(&mut self.i2c, self.addr).map_err(Error::I2c)?;
        Ok(reg)
    }

    pub fn write_register<R: prelude::Write>(&mut self, reg: R) -> Result<(), Error<E>> {
        reg.write_to_device(&mut self.i2c, self.addr).map_err(Error::I2c)?;
        Ok(())
    }

    pub fn read_manufacturer_id(&mut self) -> Result<impl ManufacturerId, Error<E>> {
        self.read_register(reg_manuf_id::new())
    }

    pub fn read_device_id(&mut self) -> Result<impl DeviceId, Error<E>> {
        self.read_register(reg_device_id::new())
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature(&mut self) -> Result<impl Temperature, Error<E>> {
        self.read_register(reg_temp::new())
    }

    pub fn read_configuration(&mut self) -> Result<impl Configuration, Error<E>> {
        self.read_register(reg_conf::new())
    }
}
