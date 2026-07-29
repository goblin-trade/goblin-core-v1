use super::MarketHeader;
use crate::{
    axis::{
        leg::{Base, Quote},
        market::{market_spec::MarketSpec, Readables, Writables},
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
        // TODO use for_axes!
        if Base::get(&self.execute_takes) {
            ix_take::<MS, Base>(ctx, readables, writables)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<MS, Quote>(ctx, readables, writables)?;
        }

        Ok(())
    }
}
