use crate::{
    axis::{leg::Pair, market::header::inner_bitmap_header::InnerBitmapHeader},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    matching::bitmap::outer_pos::OuterPos,
    require,
};

const BYTE_COUNT: usize = core::mem::size_of::<InnerBitmapHeader>();

impl Decodable for InnerBitmapHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);
        let header = Self {
            outer_pos: OuterPos::new(u8::decode_unchecked_no_advance(ctx)),
            update_count: Pair::new(
                u8::decode_unchecked_no_advance(ctx),
                u8::decode_unchecked_no_advance(ctx),
            ),
        };

        ctx.advance_offset(BYTE_COUNT);

        Ok(header)
    }
}
