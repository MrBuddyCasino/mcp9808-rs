use reg::Register;
use core::fmt::Debug;

/// Alert Output Mode bit
/// This bit cannot be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
pub enum AlertMode {
    /// power-up default
    Comparator = 0,
    Interrupt = 1,
}

/// Alert Output Polarity bit
/// This bit cannot be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
pub enum AlertPolarity {
    /// power-up default; pull-up resistor required
    ActiveLow = 0,
    ActiveHigh = 1,
}

/// Alert Output Select bit
/// When the Alarm Window Lock bit is set, this bit cannot be altered until unlocked (bit 6).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
pub enum AlertSelect {
    /// Alert output for TUPPER, TLOWER and TCRIT (power-up default)
    All = 0,
    TCritOnly = 1,
}

/// Alert Output Control bit
/// This bit can not be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
pub enum AlertControl {
    /// power-up default
    Disabled = 0,
    Enabled = 1,
}

/// Alert Output Status bit
/// This bit can not be set to ‘1’ or cleared to ‘0’ in Shutdown mode.
/// However, if the Alert output is configured as Interrupt mode, and if the host controller clears
/// to ‘0’, the interrupt, using bit 5 while the device is in Shutdown mode,
/// then this bit will also be cleared ‘0’.
pub enum AlertStatus {
    /// Alert output is not asserted by the device (power-up default)
    NotAsserted = 0,
    /// Alert output is asserted as a comparator/Interrupt or critical temperature output
    Asserted = 1,
}

/// Interrupt Clear bit
/// This bit can not be set to ‘1’ in Shutdown mode, but it can be cleared after the device enters
/// Shutdown mode.
pub enum InterruptClear {
    /// No effect (power-up default)
    NoEffect = 0,
    /// Clear interrupt output; when read, this bit returns to ‘0’
    ClearInterruptOutput = 1,
}

/// tUPPER and tLOWER Window Lock bit
/// When enabled, this bit remains set to ‘1’ or locked until cleared by a Power-on Reset.
pub enum WindowLock {
    /// tUPPER and tLOWER registers can be written (power-up default)
    Unlocked = 0,
    /// tUPPER and tLOWER registers can not be written
    Locked = 1,
}

/// When  enabled,  this  bit  remains  set  to  ‘1’  or  locked  until  cleared  by  an  internal
/// Reset.
pub enum CriticalLock {
    /// tCRIT register can be written (power-up default)
    Unlocked = 0,
    /// tCRIT register can not be written
    Locked = 1,
}

/// In shutdown, all power-consuming activities are disabled, though all registers can be written to or read.
/// This bit cannot be set to ‘1’ when either of the Lock bits is set (bit 6 and bit 7).
/// However, it can be cleared to ‘0’ for continuous conversion while locked.

pub enum ShutdownMode {
    /// Continuous conversion (power-up default)
    Continuous = 0,
    /// Shutdown (Low-Power mode)
    Shutdown = 1,
}

//noinspection RsEnumVariantNaming
/// tUPPER and tLOWER Limit Hysteresis bits
/// This bit can not be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode.
pub enum LimitHysteresis {
    Deg0_0C = 0b00,
    Deg1_5C = 0b01,
    Deg3_0C = 0b10,
    Deg6_0C = 0b11,
}

const REGISTER_PTR: u8 = 0b0001;
const REGISTER_SIZE: u8 = 2;

pub trait Configuration: Debug + Copy + Clone {
    fn get_register_ptr() -> u8;

    fn get_alert_mode(&self) -> AlertMode;
    fn set_alert_mode(&mut self, mode: AlertMode);
    fn get_alert_polarity(&self) -> AlertPolarity;
    fn set_alert_polarity(&mut self, mode: AlertPolarity);
    fn get_alert_select(&self) -> AlertSelect;
    fn set_alert_select(&mut self, mode: AlertSelect);
    fn get_alert_control(&self) -> AlertControl;
    fn set_alert_control(&mut self, mode: AlertControl);
    fn get_alert_status(&self) -> AlertStatus;
    fn set_alert_status(&mut self, mode: AlertStatus);
    fn get_interrupt_clear(&self) -> InterruptClear;
    fn set_interrupt_clear(&mut self, mode: InterruptClear);
    fn get_window_lock(&self) -> WindowLock;
    fn set_window_lock(&mut self, mode: WindowLock);
    fn get_critical_lock(&self) -> CriticalLock;
    fn set_critical_lock(&mut self, mode: CriticalLock);
    fn get_shutdown_mode(&self) -> ShutdownMode;
    fn set_shutdown_mode(&mut self, mode: ShutdownMode);
}

pub fn new() -> Register {
    Register::new(REGISTER_PTR, REGISTER_SIZE)
}

/// Sensor configuration register.
impl Configuration for Register {
    fn get_register_ptr() -> u8 {
        REGISTER_PTR
    }

    fn get_alert_mode(&self) -> AlertMode {
        if self.get_bit(0) {
            return AlertMode::Interrupt;
        }
        AlertMode::Comparator
    }

    fn set_alert_mode(&mut self, mode: AlertMode) {
        self.set_bit(0, bool(mode as isize));
    }

    fn get_alert_polarity(&self) -> AlertPolarity {
        if self.get_bit(1) {
            return AlertPolarity::ActiveHigh;
        }
        AlertPolarity::ActiveLow
    }

    fn set_alert_polarity(&mut self, mode: AlertPolarity) {
        self.set_bit(1, bool(mode as isize));
    }

    fn get_alert_select(&self) -> AlertSelect {
        if self.get_bit(2) {
            return AlertSelect::TCritOnly;
        }
        AlertSelect::All
    }

    fn set_alert_select(&mut self, mode: AlertSelect) {
        self.set_bit(2, bool(mode as isize));
    }

    fn get_alert_control(&self) -> AlertControl {
        if self.get_bit(3) {
            return AlertControl::Enabled;
        }
        AlertControl::Disabled
    }

    fn set_alert_control(&mut self, mode: AlertControl) {
        self.set_bit(3, bool(mode as isize));
    }

    fn get_alert_status(&self) -> AlertStatus {
        if self.get_bit(4) {
            return AlertStatus::Asserted;
        }
        AlertStatus::NotAsserted
    }

    fn set_alert_status(&mut self, mode: AlertStatus) {
        self.set_bit(4, bool(mode as isize));
    }

    fn get_interrupt_clear(&self) -> InterruptClear {
        if self.get_bit(5) {
            return InterruptClear::ClearInterruptOutput;
        }
        InterruptClear::NoEffect
    }

    fn set_interrupt_clear(&mut self, mode: InterruptClear) {
        self.set_bit(5, bool(mode as isize));
    }

    fn get_window_lock(&self) -> WindowLock {
        if self.get_bit(6) {
            return WindowLock::Locked;
        }
        WindowLock::Unlocked
    }

    fn set_window_lock(&mut self, mode: WindowLock) {
        self.set_bit(6, bool(mode as isize));
    }

    fn get_critical_lock(&self) -> CriticalLock {
        if self.get_bit(7) {
            return CriticalLock::Locked;
        }
        CriticalLock::Unlocked
    }

    fn set_critical_lock(&mut self, mode: CriticalLock) {
        self.set_bit(7, bool(mode as isize));
    }

    fn get_shutdown_mode(&self) -> ShutdownMode {
        if self.get_bit(8) {
            return ShutdownMode::Shutdown;
        }
        ShutdownMode::Continuous
    }

    fn set_shutdown_mode(&mut self, mode: ShutdownMode) {
        self.set_bit(8, bool(mode as isize));
    }
}

/// dirty little helper where 0 is false, > 0 is true
fn bool(val: isize) -> bool {
    val != 0
}