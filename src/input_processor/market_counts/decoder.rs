use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, MarketCounts},
    require,
};

const BYTE_COUNT: usize = 3;

impl<'a> Decodable<'a> for MarketCounts {
    fn decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = ctx.decode_unchecked_no_advance::<u8>();
        let byte_1 = ctx.decode_unchecked_no_advance::<u8>();
        let byte_2 = ctx.decode_unchecked_no_advance::<u8>();
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
