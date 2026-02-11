use crate::{
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, GlobalHeader},
    market::market_counts::MarketCounts,
    settlement::Delta,
    types::{Address, Dynamic, Hardcoded, StoreReader},
};

impl<'a> MarketCounts for GlobalHeader<'a> {
    fn process(
        &self,
        ctx: &DecodeCtx,
        msg_sender: &Address,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        let hardcoded_counts = Hardcoded::get_leg(&self.market_counts);
        hardcoded_counts.process(ctx, msg_sender, delta)?;

        if let Some(dynamic_counts) = Dynamic::get_leg(&self.market_counts) {
            dynamic_counts.process(ctx, msg_sender, delta)?;
        }
        Ok(())
    }
}
