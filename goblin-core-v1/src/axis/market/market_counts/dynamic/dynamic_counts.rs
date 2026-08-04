use crate::{
    goblin_error::GoblinError,
    input_processor::{DecodablePrimitive, DecodeCtx},
    require,
};

const BYTE_COUNT: usize = 4;

/// The number of dynamic markets to process and their associated custom token addresses
#[derive(Clone, Copy, Default)]
pub struct DynamicCounts {
    pub market_counts: [u8; 8],
}

// TODO remove
impl DynamicCounts {
    pub fn new(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let byte_1 = u8::decode_unchecked_no_advance(ctx);
        let byte_2 = u8::decode_unchecked_no_advance(ctx);
        let byte_3 = u8::decode_unchecked_no_advance(ctx);

        let market_counts = [
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
            byte_3 & 0b0000_1111,
            byte_3 >> 4,
        ];

        ctx.advance_offset(BYTE_COUNT);

        Ok(Self { market_counts })
    }
}
