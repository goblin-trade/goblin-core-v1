use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx, MarketCounts},
    require,
};

const BYTE_COUNT: usize = 3;

impl<'a> Decodable<'a> for MarketCounts {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let byte_1 = u8::decode_unchecked_no_advance(ctx);
        let byte_2 = u8::decode_unchecked_no_advance(ctx);

        ctx.advance_offset(BYTE_COUNT);

        Ok(Self::new([
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
        ]))
    }
}
