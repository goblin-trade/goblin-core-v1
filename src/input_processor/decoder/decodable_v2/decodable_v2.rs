use crate::{goblin_error::GoblinError, input_processor::DecodeCtx, require};

pub trait DecodableV2: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const SIZE: usize;

    /// Pure function of (ctx, offset) — does NOT touch ctx.offset.
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self;

    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let offset = ctx.offset.get();
        require!(
            ctx.len() >= offset + Self::SIZE,
            GoblinError::InvalidPayload
        );
        let value = Self::decode_unchecked(ctx, offset);
        ctx.advance_offset(Self::SIZE);
        Ok(value)
    }
}
