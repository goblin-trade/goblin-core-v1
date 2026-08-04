use crate::{goblin_error::GoblinError, input_processor::DecodeCtx, require};

/// Decode values from a byte buffer borrowed for lifetime `'a`.
///
/// The `'a` parameter ties the lifetime of the decoded value to the lifetime
/// of the `DecodeCtx` reference it was decoded from. Types that own their
/// data (all integers, fixed byte arrays, etc.) simply ignore `'a` and
/// implement `FixedDecode<'a>` for every `'a`. Types that borrow directly
/// from the underlying buffer (e.g. `GlobalHeader<'a>`) tie their own
/// lifetime parameter to it.
///
/// `decode_raw` MUST advance `ctx`'s offset by exactly `ENCODED_SIZE` bytes
/// before returning, so that struct fields decoded in sequence naturally
/// read from the correct positions.
pub trait FixedDecode<'a>: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const ENCODED_SIZE: usize;

    /// Decode assuming `ENCODED_SIZE` bytes are available at the current
    /// offset. Advances `ctx`'s offset by `ENCODED_SIZE`.
    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self;

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_fixed_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + Self::ENCODED_SIZE,
            GoblinError::InvalidPayload
        );
        Ok(Self::raw_fixed_decode(ctx))
    }
}
