use crate::{goblin_error::GoblinError, input_processor::DecodeCtx, require};

/// Decode fixed-size values from a byte buffer.
///
/// All implementors have a known, constant encoded size (`SIZE`) and can be
/// decoded without per-field bounds checks once the caller has verified
/// enough bytes remain (see `try_decode`, which does that check for you).
///
/// `decode_unchecked` MUST advance `ctx`'s offset by exactly `SIZE` bytes
/// before returning, so that struct fields decoded in sequence naturally
/// read from the correct positions.
pub trait DecodableV2: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const ENCODED_SIZE: usize;

    /// Decode assuming `SIZE` bytes are available at the current offset.
    /// Advances `ctx`'s offset by `SIZE`.
    fn decode_raw(ctx: &DecodeCtx) -> Self;

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + Self::ENCODED_SIZE,
            GoblinError::InvalidPayload
        );
        Ok(Self::decode_raw(ctx))
    }
}
