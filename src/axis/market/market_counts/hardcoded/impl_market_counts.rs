use super::HardcodedCounts;
use crate::{
    axis::{
        market::{market_counts::MarketCounts, process_market, Hardcoded},
        token::{token_list::custom_erc20::CustomERC20List, HardcodedERC20, ETH},
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
        custom_erc20_list: CustomERC20List<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.inner[0] {
            process_market::<Hardcoded, ETH, HardcodedERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[1] {
            process_market::<Hardcoded, HardcodedERC20, ETH>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[2] {
            process_market::<Hardcoded, HardcodedERC20, HardcodedERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        Ok(())
    }
}
