use crate::{
    axis::{
        market::{
            header::market_header::{
                execute_makes::process_outer_bitmap::process_outer_bitmap, MarketHeader,
            },
            market_marker::MarketMarker,
            MarketAndKey,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::local_delta::LocalDelta,
    state::MarketState,
    types::Address,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_makes(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            process_outer_bitmap(msg_sender, ctx, local_delta, market_and_key, market_state)?;
        }

        Ok(())
    }
}
