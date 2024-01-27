#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate cast;
extern crate embedded_hal;

use crate::error::Error;
use crate::reg_conf::Configuration;
use crate::reg_device_id::DeviceId;
use crate::reg_manuf_id::ManufacturerId;
use crate::reg_res::Resolution;
use crate::reg_temp::Temperature;
use crate::reg_temp_alert_crit::CriticalTemperatureAlert;
use crate::reg_temp_alert_lower::LowerTemperatureAlert;
use crate::reg_temp_alert_upper::UpperTemperatureAlert;
use crate::address::SlaveAddress;
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub mod error;
mod prelude;
pub mod address;
pub mod reg;
pub mod reg_conf;
pub mod reg_device_id;
pub mod reg_manuf_id;
pub mod reg_res;
pub mod reg_temp;
pub mod reg_temp_alert_crit;
pub mod reg_temp_alert_lower;
pub mod reg_temp_alert_upper;
pub mod reg_temp_generic;

/// MCP9808 Driver
pub struct MCP9808<I2C> {
    addr: u8,
    i2c: I2C,
}

impl<I2C> MCP9808<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    /// Creates a new driver from an I2C peripheral.
    pub fn new(i2c: I2C, addr: SlaveAddress) -> Self {
        MCP9808 {
            addr: addr.into(),
            i2c,
        }
    }

    /// release resources
    pub fn free(self) -> I2C {
        self.i2c
    }

    fn read_register<T>(&mut self, mut reg: T) -> Result<T, Error<I2C::Error>>
    where
        T: prelude::Read,
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        reg.read_from_device(&mut self.i2c, self.addr)?;
        Ok(reg)
    }

    pub fn write_register<R: prelude::Write>(&mut self, reg: R) -> Result<(), Error<I2C::Error>> {
        reg.write_to_device(&mut self.i2c, self.addr)?;
        Ok(())
    }

    pub fn read_configuration(&mut self) -> Result<impl Configuration, Error<I2C::Error>> {
        self.read_register(reg_conf::new())
    }

    pub fn read_device_id(&mut self) -> Result<impl DeviceId, Error<I2C::Error>> {
        self.read_register(reg_device_id::new())
    }

    pub fn read_manufacturer_id(&mut self) -> Result<impl ManufacturerId, Error<I2C::Error>> {
        self.read_register(reg_manuf_id::new())
    }

    pub fn read_resolution(&mut self) -> Result<impl Resolution, Error<I2C::Error>> {
        self.read_register(reg_res::new())
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature(&mut self) -> Result<impl Temperature, Error<I2C::Error>> {
        self.read_register(reg_temp::new())
    }

    pub fn read_alert_critical(
        &mut self,
    ) -> Result<impl CriticalTemperatureAlert, Error<I2C::Error>> {
        self.read_register(reg_temp_alert_crit::new())
    }

    pub fn read_alert_lower(&mut self) -> Result<impl LowerTemperatureAlert, Error<I2C::Error>> {
        self.read_register(reg_temp_alert_lower::new())
    }

    pub fn read_alert_upper(&mut self) -> Result<impl UpperTemperatureAlert, Error<I2C::Error>> {
        self.read_register(reg_temp_alert_upper::new())
    }
}
