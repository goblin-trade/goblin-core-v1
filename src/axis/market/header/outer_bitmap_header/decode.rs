use crate::{
    axis::market::header::outer_bitmap_header::OuterBitmapHeader,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
    require,
};

const BYTE_COUNT: usize = core::mem::size_of::<OuterBitmapHeader>();

impl Decodable for OuterBitmapHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);
        let header = Self {
            outer_bitmap_index: OuterBitmapIndex::new(u64::decode_unchecked_no_advance(ctx)),
            inner_bitmap_count: u8::decode_unchecked_no_advance(ctx),
        };

        ctx.advance_offset(BYTE_COUNT);

        Ok(header)
    }
}
