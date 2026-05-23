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
        readables: &Readables<M, B, Q>,
        writables: &mut Writables,
    ) -> Result<(), GoblinError> {
        if Base::get(&self.execute_takes) {
            ix_take::<M, B, Q, Base>(ctx, readables, writables)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<M, B, Q, Quote>(ctx, readables, writables)?;
        }

        Ok(())
    }
}
