use crate::{goblin_error::GoblinError, input_processor::ArgsReader, require};

pub trait VariableDecode<'a>: Sized {
    type Flags;

    fn size(flags: &Self::Flags) -> usize;

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self;

    /// Check constraints on an already-decoded value.
    /// Default is a no-op; leaf types override it.
    fn validate(&self) -> Result<(), GoblinError> {
        Ok(())
    }

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_variable_decode(
        reader: &'a ArgsReader,
        flags: &Self::Flags,
    ) -> Result<Self, GoblinError> {
        require!(
            reader.len() >= reader.offset.get() + Self::size(flags),
            GoblinError::InvalidPayload
        );
        let value = Self::raw_variable_decode(reader, flags);
        value.validate()?;
        Ok(value)
    }
}
