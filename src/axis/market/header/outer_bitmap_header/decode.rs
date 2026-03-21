use crate::{
    axis::market::header::outer_bitmap_header::OuterBitmapHeader,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl Decodable for OuterBitmapHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        Ok(Self {
            inner_bitmap_count: u8::try_decode(ctx)?,
        })
    }
}
