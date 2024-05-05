//! Device Address

const DEFAULT_ADDRESS: u8 = 0b1_1000;

/// I2C device address
///
/// # Example
///
/// ## Default slave address
/// ```
/// use mcp9808::address::SlaveAddress;
///
/// # let DefaultAddress = SlaveAddress::Default.into();
/// assert_eq!(0b1_1000u8, DefaultAddress);
/// ```
///
/// ## Alternative slave address
/// ```
/// use mcp9808::address::SlaveAddress;
///
/// # let TestAddress = SlaveAddress::Alternative { a2: true, a1: false, a0: true }.into();
/// assert_eq!(0b1_1101u8, TestAddress);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddress {
    /// Default slave address 0b1_1000 - all pins are disconnected
    Default,
    /// Alternative slave address with configurable pins A2, A1, A0.
    /// The address pins correspond to the Least Significant
    /// bits (LSbs) of the address bits.
    Alternative { a2: bool, a1: bool, a0: bool },
}

impl From<SlaveAddress> for u8 {
    fn from(slave_address: SlaveAddress) -> Self {
        match slave_address {
            SlaveAddress::Default => DEFAULT_ADDRESS,
            SlaveAddress::Alternative { a2, a1, a0 } => {
                DEFAULT_ADDRESS | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8)
            }
        }
    }
}

impl PartialEq for SlaveAddress {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs): (u8, u8) = ((*self).into(), (*other).into());
        lhs == rhs
    }
}
