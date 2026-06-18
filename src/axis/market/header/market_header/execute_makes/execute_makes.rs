use crate::{
    axis::{
        market::{
            header::{market_header::MarketHeader, outer_bitmap_header::OuterBitmapHeader},
            market_marker::MarketMarker,
            Readables, Writables,
        },
        token::token_reader::TokenReader,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub fn execute_makes(
        &self,
        ctx: &DecodeCtx,
        readables: &Readables<M, B, Q>,
        writables: &mut Writables,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            OuterBitmapHeader::process(ctx, readables, writables)?;
        }

        Ok(())
    }
}
