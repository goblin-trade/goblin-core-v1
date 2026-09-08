mod impl_fixed_decode;

use crate::{goblin_error::GoblinError, input_processor::ArgsReader, require};

/// Decode values from a fixed sized buffer
pub trait FixedDecode<'a>: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const ENCODED_SIZE: usize;

    /// Decode assuming `ENCODED_SIZE` bytes are available at the current
    /// offset. Advances `reader`'s offset by `ENCODED_SIZE`.
    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self;

    /// Check constraints on an already-decoded value.
    /// Default is a no-op; leaf types override it.
    fn validate(&self) -> Result<(), GoblinError> {
        Ok(())
    }

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_fixed_decode(reader: &'a ArgsReader) -> Result<Self, GoblinError> {
        require!(
            reader.len() >= reader.offset.get() + Self::ENCODED_SIZE,
            GoblinError::InvalidPayload
        );
        let value = Self::raw_fixed_decode(reader);
        value.validate()?;
        Ok(value)
    }
}
