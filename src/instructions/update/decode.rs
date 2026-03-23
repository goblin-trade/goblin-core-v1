use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    instructions::update_header::UpdateHeader,
    quantities::BaseLots,
    require,
};

const BYTE_COUNT: usize = core::mem::size_of::<UpdateHeader>();

impl Decodable for UpdateHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);
        let header = Self {
            base_lots: BaseLots::new(u64::decode_unchecked_no_advance(ctx)),
        };

        ctx.advance_offset(BYTE_COUNT);

        Ok(header)
    }
}
