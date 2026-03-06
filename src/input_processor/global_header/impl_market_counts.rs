use crate::{
    axis::market::{market_counts::MarketCounts, Dynamic, Hardcoded},
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, GlobalHeader},
    settlement::Delta,
    types::StoreReader,
};

impl<'a> MarketCounts for GlobalHeader<'a> {
    fn process(&self, ctx: &DecodeCtx, delta: &mut Delta) -> Result<(), GoblinError> {
        let hardcoded_counts = Hardcoded::get_leg(&self.market_counts);
        hardcoded_counts.process(ctx, delta)?;

        if let Some(dynamic_counts) = Dynamic::get_leg(&self.market_counts) {
            dynamic_counts.process(ctx, delta)?;
        }
        Ok(())
    }
}
