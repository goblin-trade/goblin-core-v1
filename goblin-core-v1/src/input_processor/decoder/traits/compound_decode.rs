use crate::{goblin_error::GoblinError, input_processor::DecodeCtx};

/// Flexible trait to decode values from ctx. The value can be compound-
/// containing `FixedDecode`, `VariableDecode` and non buffer sources like Hostio.
pub trait CompoundDecode<'a>: Sized {
    fn try_compound_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError>;
}
