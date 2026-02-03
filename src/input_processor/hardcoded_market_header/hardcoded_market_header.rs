use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{process_market, Dynamic, Hardcoded},
    settlement::Delta,
    token::{CustomERC20, CustomERC20Data, HardcodedERC20, ETH},
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

        // // Dynamic with hardcoded ERC20 (3)
        // for _ in 0..self.inner[3] {
        //     process_market::<Dynamic, ETH, HardcodedERC20>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        // for _ in 0..self.inner[4] {
        //     process_market::<Dynamic, HardcodedERC20, ETH>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        // for _ in 0..self.inner[5] {
        //     process_market::<Dynamic, HardcodedERC20, HardcodedERC20>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        // // Dynamic with custom ERC20 (3)
        // for _ in 0..self.inner[6] {
        //     process_market::<Dynamic, ETH, CustomERC20>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }

        // for _ in 0..self.inner[7] {
        //     process_market::<Dynamic, CustomERC20, ETH>(ctx, msg_sender, custom_erc20_list, delta)?;
        // }

        // for _ in 0..self.inner[8] {
        //     process_market::<Dynamic, CustomERC20, CustomERC20>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        // // Dynamic with mixture of hardcoded and custom ERC20 (2)
        // for _ in 0..self.inner[9] {
        //     process_market::<Dynamic, HardcodedERC20, CustomERC20>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        // for _ in 0..self.inner[10] {
        //     process_market::<Dynamic, CustomERC20, HardcodedERC20>(
        //         ctx,
        //         msg_sender,
        //         custom_erc20_list,
        //         delta,
        //     )?;
        // }

        Ok(())
    }
}
