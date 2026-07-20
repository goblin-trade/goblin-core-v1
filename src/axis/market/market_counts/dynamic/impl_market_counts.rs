use super::DynamicCounts;
use crate::{
    axis::{
        market::{market_counts::MarketCounts, process_market, Dynamic},
        token::{token_list::custom_erc20::CustomERC20List, CustomERC20, HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::Delta,
    types::Address,
};

impl MarketCounts for DynamicCounts {
    fn process<'a>(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        custom_erc20_list: CustomERC20List<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        // Dynamic with hardcoded ERC20 (3)
        for _ in 0..self.market_counts[0] {
            process_market::<Dynamic, ETH, HardcodedERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[1] {
            process_market::<Dynamic, HardcodedERC20, ETH>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[2] {
            process_market::<Dynamic, HardcodedERC20, HardcodedERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        // Dynamic with custom ERC20 (3)
        for _ in 0..self.market_counts[3] {
            process_market::<Dynamic, ETH, CustomERC20>(msg_sender, ctx, custom_erc20_list, delta)?;
        }

        for _ in 0..self.market_counts[4] {
            process_market::<Dynamic, CustomERC20, ETH>(msg_sender, ctx, custom_erc20_list, delta)?;
        }

        for _ in 0..self.market_counts[5] {
            process_market::<Dynamic, CustomERC20, CustomERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        // Dynamic with mixture of hardcoded and custom ERC20 (2)
        for _ in 0..self.market_counts[6] {
            process_market::<Dynamic, HardcodedERC20, CustomERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[7] {
            process_market::<Dynamic, CustomERC20, HardcodedERC20>(
                msg_sender,
                ctx,
                custom_erc20_list,
                delta,
            )?;
        }

        Ok(())
    }
}
