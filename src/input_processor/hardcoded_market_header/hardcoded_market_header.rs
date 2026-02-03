use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{process_market, Hardcoded},
    settlement::Delta,
    token::{HardcodedERC20, ETH},
    types::Address,
};

/// The number of markets of each type
pub struct HardcodedMarketHeader {
    inner: [u8; 3],
}

impl HardcodedMarketHeader {
    pub fn new(inner: [u8; 3]) -> Self {
        Self { inner }
    }

    /// Process legal combinations of market types
    pub fn process_markets(
        &self,
        ctx: &DecodeCtx,
        msg_sender: &Address,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        // Hardcoded (3)
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
