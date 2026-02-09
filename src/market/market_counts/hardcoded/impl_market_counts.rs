use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{process_market, HardcodedCounts, MarketCounts},
    settlement::Delta,
    types::{Address, Hardcoded, HardcodedERC20, ETH},
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
