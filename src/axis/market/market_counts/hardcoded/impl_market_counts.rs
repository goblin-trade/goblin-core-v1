use super::HardcodedCounts;
use crate::{
    axis::{
        market::{market_counts::MarketCounts, process_market, Hardcoded},
        token::{HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::Delta,
    types::Address,
};

impl MarketCounts for HardcodedCounts {
    fn process(
        &self,
        ctx: &DecodeCtx,
        msg_sender: &Address,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.inner[0] {
            process_market::<Hardcoded, ETH, HardcodedERC20>(ctx, msg_sender, (), delta)?;
        }

        for _ in 0..self.inner[1] {
            process_market::<Hardcoded, HardcodedERC20, ETH>(ctx, msg_sender, (), delta)?;
        }

        for _ in 0..self.inner[2] {
            process_market::<Hardcoded, HardcodedERC20, HardcodedERC20>(
                ctx,
                msg_sender,
                (),
                delta,
            )?;
        }

        Ok(())
    }
}
