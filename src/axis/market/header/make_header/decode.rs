use crate::{
    axis::market::header::make_header::MakeHeader,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    instructions::make_variant::MakeVariant,
    quantities::{BaseLots, InnerPosV2},
    require,
};

const BYTE_COUNT: usize = 1 + 8;

impl Decodable for MakeHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);

        let inner_pos = InnerPosV2::new(u8::decode_unchecked_no_advance(ctx));
        let bytes = u64::decode_unchecked_no_advance(ctx);

        let make_variant = MakeVariant::from(bytes);

        let base_lots = BaseLots::new(bytes >> 2);

        let header = Self {
            inner_pos,
            base_lots,
            make_variant,
        };

        ctx.advance_offset(BYTE_COUNT);

        Ok(header)
    }
}
