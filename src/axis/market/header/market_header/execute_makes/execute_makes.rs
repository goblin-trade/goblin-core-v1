use crate::{
    axis::{
        market::{
            header::{market_header::MarketHeader, outer_bitmap_header::OuterBitmapHeader},
            market_marker::MarketMarker,
            Readables,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::local_delta::LocalDelta,
    state::MarketState,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_makes(
        &self,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_state: &mut MarketState,
        readables: &Readables<M, B, Q>,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            OuterBitmapHeader::process(ctx, local_delta, market_state, readables)?;
        }

        Ok(())
    }
}
