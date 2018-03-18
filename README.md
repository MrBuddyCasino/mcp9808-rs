# mcp9808-rs

# Rust MCP9808 Driver

This is a platform agnostic Rust driver for the MCP9808, based on the
[`embedded-hal`](https://github.com/japaric/embedded-hal) traits.


## The Device

The MCP9808 digital temperature sensor converts temperatures between -20°C and +100°C to a 
digital word with ±0.5°C (max.) accuracy.

The device has an I²C interface and user-selectable settings such as Shutdown or 
low-power modes and the specification of temperature Event and Critical output boundaries.

Details and datasheet: http://www.microchip.com/wwwproducts/en/en556182


## Status
Feature complete, but needs more testing.
- [x] temperature register
- [x] configuration register
- [x] resolution register
- [x] manufacturer ID and device ID
- [x] temperature alert upper & lower
- [x] critical temperature


## Usage

Assuming you have a reference to the HAL's I2C bus:

```
    let mut mcp9808 = MCP9808::new(i2c);

    // how to read & write register
    let mut conf = mcp9808.read_configuration().unwrap();
    conf.set_shutdown_mode(ShutdownMode::Shutdown);
    let _c = mcp9808.write_register(conf);
    
    // read temperature register
    let temp = mcp9808.read_temperature().unwrap();
    temp.get_celcius(ResolutionVal::Deg_0_0625C)
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.


### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.