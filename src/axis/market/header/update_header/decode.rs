use crate::{
    axis::{market::header::update_header::UpdateHeader, update::UpdateEnum},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    matching::bitmap::inner_pos::InnerPos,
    quantities::BaseLots,
    require,
};

const BYTE_COUNT: usize = 1 + 64;

impl Decodable for UpdateHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);

        let inner_pos = InnerPos::new(u8::decode_unchecked_no_advance(ctx));
        let bytes = u64::decode_unchecked_no_advance(ctx);

        let update_variant_raw = 0b0000_0001 & bytes == 1;
        let update_variant = UpdateEnum::from(update_variant_raw);

        let base_lots = BaseLots::new(bytes >> 1);

        let header = Self {
            inner_pos,
            base_lots,
            update_variant,
        };

        ctx.advance_offset(BYTE_COUNT);

        Ok(header)
    }
}
