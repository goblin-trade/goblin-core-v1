use crate::{axis::token::token_marker::TokenMarker, goblin_error::GoblinError};

/// Trait to map runtime `decimals:u8` to legal generic decimal constants.
///
/// This is the canonical store of legal decimal places
pub trait DecimalAction<T: TokenMarker> {
    /// The function to be called
    fn run<const D: u8>(&self) -> Result<(), GoblinError>;

    /// Match `run()` against legal decimal places and run it
    /// if decimals is valid
    fn dispatch(&self, decimals: T::StoredDecimals) -> Result<(), GoblinError> {
        match decimals.into() {
            6 => self.run::<6>(),
            8 => self.run::<8>(),
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
