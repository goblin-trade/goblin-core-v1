use crate::{goblin_error::GoblinError, input_processor::ArgsReader};

/// Flexible trait to decode values from reader. The value can be compound-
/// containing `FixedDecode`, `VariableDecode` and non buffer sources like Hostio.
pub trait CompoundDecode<'a>: Sized {
    fn try_compound_decode(reader: &'a ArgsReader) -> Result<Self, GoblinError>;
}
