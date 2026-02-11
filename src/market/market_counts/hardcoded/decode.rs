use super::HardcodedCounts;
use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    require,
};

const BYTE_COUNT: usize = 2;

impl Decodable for HardcodedCounts {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let byte_1 = u8::decode_unchecked_no_advance(ctx);
        ctx.advance_offset(BYTE_COUNT);

        Ok(Self::new([
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
        ]))
    }
}
