use crate::{goblin_error::GoblinError, input_processor::DecodeCtx, require};

/// Decode values from a fixed sized buffer
pub trait FixedDecode<'a>: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const ENCODED_SIZE: usize;

    /// Decode assuming `ENCODED_SIZE` bytes are available at the current
    /// offset. Advances `ctx`'s offset by `ENCODED_SIZE`.
    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self;

    /// Check constraints on an already-decoded value.
    /// Default is a no-op; leaf types override it.
    fn validate(&self) -> Result<(), GoblinError> {
        Ok(())
    }

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_fixed_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + Self::ENCODED_SIZE,
            GoblinError::InvalidPayload
        );
        let value = Self::raw_fixed_decode(ctx);
        value.validate()?;
        Ok(value)
    }
}
