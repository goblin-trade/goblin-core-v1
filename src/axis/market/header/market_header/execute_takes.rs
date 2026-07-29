use super::MarketHeader;
use crate::{
    axis::{
        leg::{Base, Quote},
        market::{market_marker::MarketMarker, market_spec::MarketSpec, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    instructions::ix_take,
    types::StoreReader,
};

impl<MS: MarketSpec> MarketHeader<MS> {
    pub fn execute_takes(
        &self,
        ctx: &DecodeCtx,
        readables: &Readables<MS>,
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
