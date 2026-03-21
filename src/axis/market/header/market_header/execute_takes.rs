use super::MarketHeader;
use crate::{
    axis::{
        leg::{Base, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    instructions::ix_take,
    settlement::local_delta::LocalDelta,
    state::MarketState,
    types::StoreReader,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_takes(
        &self,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState<M, B, Q>,
    ) -> Result<(), GoblinError> {
        if Base::get(&self.execute_takes) {
            ix_take::<M, B, Q, Base>(ctx, local_delta, market_and_key, market_state)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<M, B, Q, Quote>(ctx, local_delta, market_and_key, market_state)?;
        }

        Ok(())
    }
}
