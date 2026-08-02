use crate::{
    goblin_error::GoblinError,
    input_processor::{DecodablePrimitive, DecodeCtx},
    require,
};

/// Trait to attempt decoding values from DecodeCtx
pub trait Decodable: Sized {
    /// Try to decode
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError>;
}

/// Blanket implementation for primitive decodables
impl<K: DecodablePrimitive> Decodable for K {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let offset = ctx.offset.get();
        let size = core::mem::size_of::<Self>();
        require!(ctx.len() >= offset + size, GoblinError::InvalidPayload);

        let value = Self::decode_unchecked_no_advance(ctx);
        ctx.advance_offset(size);

        Ok(value)
    }
}
