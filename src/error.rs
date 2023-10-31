/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
    RegisterSizeMismatch(u8),
}
impl<E> From<E> for Error<E> {
    fn from(other: E) -> Self {
        Error::I2c(other)
    }
}
