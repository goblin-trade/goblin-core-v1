use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    instructions::ix_take,
    market::{CommonMarket, MarketHeader, MarketVariant},
    settlement::local_delta::LocalDelta,
    state::MarketState,
    token::TokenMarker,
    types::{Base, Quote, TupleReader},
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketVariant,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_takes(
        &self,
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        local_delta: &mut LocalDelta,
        common_market: &CommonMarket<M, B, Q>,
        market_state: &mut MarketState<M, B, Q>,
    ) -> Result<(), GoblinError> {
        if Base::get(&self.execute_takes) {
            ix_take::<M, B, Q, Base>(ctx, offset, len, local_delta, common_market, market_state)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<M, B, Q, Quote>(ctx, offset, len, local_delta, common_market, market_state)?;
        }

        Ok(())
    }
}
