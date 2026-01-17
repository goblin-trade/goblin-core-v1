use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{process_market, Dynamic, Hardcoded},
    settlement::Delta,
    token::{CustomERC20, CustomERC20Data, HardcodedERC20, ERC20, ETH},
    types::Address,
};

/// The number of markets of each type
pub struct MarketCounts {
    inner: [u8; 11],
}

impl MarketCounts {
    pub fn new(inner: [u8; 11]) -> Self {
        Self { inner }
    }

    /// Process legal combinations of market types
    pub fn process_markets<'a>(
        &self,
        ctx: &'a DecodeCtx<'a>,
        msg_sender: &Address,
        custom_erc20_list: &[CustomERC20Data],
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        // Hardcoded- 3
        for _ in 0..self.inner[0] {
            process_market::<Hardcoded, ETH, HardcodedERC20>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[1] {
            process_market::<Hardcoded, HardcodedERC20, ETH>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[2] {
            process_market::<Hardcoded, HardcodedERC20, HardcodedERC20>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }

        // TODO 8 dynamic variants

        // 3- with hardcoded tokens
        for _ in 0..self.inner[3] {
            process_market::<Dynamic, ETH, HardcodedERC20>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[4] {
            process_market::<Dynamic, HardcodedERC20, ETH>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.inner[5] {
            process_market::<Dynamic, HardcodedERC20, HardcodedERC20>(
                ctx,
                msg_sender,
                custom_erc20_list,
                delta,
            )?;
        }
        // 5- with hardcoded and custom mixed

        // for _ in 0..self.inner[6] {
        //     process_market::<Dynamic, ETH, CustomERC20>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }

        // Dynamic
        // for _ in 0..self.inner[3] {
        //     process_market::<Dynamic, ETH, ERC20>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }
        // for _ in 0..self.inner[4] {
        //     process_market::<Dynamic, ERC20, ETH>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }
        // for _ in 0..self.inner[5] {
        //     process_market::<Dynamic, ERC20, ERC20>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }

        Ok(())
    }
}
