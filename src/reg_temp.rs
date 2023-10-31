use reg::Register;
use reg_temp_generic::ReadableTempRegister;

const REGISTER_PTR: u8 = 0b0101;
const REGISTER_SIZE: u8 = 2;

const BIT_ALERT_CRITICAL: usize = 15;
const BIT_ALERT_UPPER: usize = 14;
const BIT_ALERT_LOWER: usize = 13;

pub trait Temperature: ReadableTempRegister {
    fn is_alert_critical(&self) -> bool;
    fn is_alert_upper(&self) -> bool;
    fn is_alert_lower(&self) -> bool;
}

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

impl Temperature for Register {
    fn is_alert_critical(&self) -> bool {
        self.get_bit(BIT_ALERT_CRITICAL)
    }
    fn is_alert_upper(&self) -> bool {
        self.get_bit(BIT_ALERT_UPPER)
    }
    fn is_alert_lower(&self) -> bool {
        self.get_bit(BIT_ALERT_LOWER)
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    /// prevent auto-format fuckup
    use super::*;

    #[test]
    fn alert_critical() {
        let mut reg = new();

        assert_eq!(reg.is_alert_critical(), false);
        reg.set_bit(BIT_ALERT_CRITICAL, true);
        assert_eq!(reg.is_alert_critical(), true);
    }

    #[test]
    fn alert_upper() {
        let mut reg = new();

        assert_eq!(reg.is_alert_upper(), false);
        reg.set_bit(BIT_ALERT_UPPER, true);
        assert_eq!(reg.is_alert_upper(), true);
    }

    #[test]
    fn alert_lower() {
        let mut reg = new();

        assert_eq!(reg.is_alert_lower(), false);
        reg.set_bit(BIT_ALERT_LOWER, true);
        assert_eq!(reg.is_alert_lower(), true);
    }
}
