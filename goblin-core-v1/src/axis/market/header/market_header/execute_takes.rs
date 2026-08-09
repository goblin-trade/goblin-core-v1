use super::MarketHeader;
use crate::{
    axis::market::{market_spec::MarketSpec, Readables, Writables},
    for_axes,
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
        for_axes!(|In| {
            if In::get(&self.execute_takes) {
                ix_take::<MS, In>(ctx, readables, writables)?;
            }
        });

        Ok(())
    }
}
