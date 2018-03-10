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
Work In Progress
- [x] temperature register, though slightly imprecise
- [x] configuration register
- [x] resolution register
- [x] manufacturer ID and device ID
- [ ] temperature alert upper & lower
- [ ] critical temperature


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