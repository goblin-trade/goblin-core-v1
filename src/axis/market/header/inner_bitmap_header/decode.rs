use crate::{
    axis::market::header::inner_bitmap_header::InnerBitmapHeader,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl Decodable for InnerBitmapHeader {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        Ok(Self {
            update_count: u8::try_decode(ctx)?,
        })
    }
}
