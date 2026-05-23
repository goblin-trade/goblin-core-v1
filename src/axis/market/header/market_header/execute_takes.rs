use super::MarketHeader;
use crate::{
    axis::{
        leg::{Base, Quote},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    instructions::ix_take,
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
        writables: &mut Writables,
        readables: &Readables<M, B, Q>,
    ) -> Result<(), GoblinError> {
        if Base::get(&self.execute_takes) {
            ix_take::<M, B, Q, Base>(ctx, writables, readables)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<M, B, Q, Quote>(ctx, writables, readables)?;
        }

        Ok(())
    }
}
