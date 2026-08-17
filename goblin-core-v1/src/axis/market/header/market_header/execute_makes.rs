use crate::{
    axis::market::{
        header::{market_header::MarketHeader, outer_bitmap_header::OuterBitmapHeader},
        Readables, Writables,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
};

impl<MS: MarketSpec> MarketHeader<MS> {
    pub fn execute_makes(
        &self,
        ctx: &DecodeCtx,
        readables: &Readables<MS>,
        writables: &mut Writables,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            OuterBitmapHeader::process(ctx, readables, writables)?;
        }

        Ok(())
    }
}
