use super::HardcodedCounts;
use crate::{
    axis::{
        leg::Pair,
        market::{market_counts::MarketCounts, process_market, Hardcoded},
        token::{token_reader::TokenDataTriple, HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::Delta,
    types::Address,
};

impl MarketCounts for HardcodedCounts {
    fn process<'a>(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.inner[0] {
            process_market::<Hardcoded, Pair<ETH, HardcodedERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..self.inner[1] {
            process_market::<Hardcoded, Pair<HardcodedERC20, ETH>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..self.inner[2] {
            process_market::<Hardcoded, Pair<HardcodedERC20, HardcodedERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        Ok(())
    }
}
