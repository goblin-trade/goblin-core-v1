use crate::goblin_error::GoblinError;

/// Trait to map runtime `decimals:u8` to legal generic decimal constants.
///
/// This is the canonical store of legal decimal places
pub trait DecimalAction {
    /// The function to be called
    fn run<const D: u8>(&self) -> Result<(), GoblinError>;

    /// Match `run()` against legal decimal places and run it
    /// if decimals is valid
    fn dispatch(&self, decimals: u8) -> Result<(), GoblinError> {
        match decimals {
            6 => self.run::<6>(),
            8 => self.run::<8>(),
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
