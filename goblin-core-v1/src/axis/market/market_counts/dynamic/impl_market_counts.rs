use crate::{
    axis::{
        leg::Pair,
        market::{
            market_counts::{MarketCounts, MarketCountsTuple},
            process_market, Dynamic,
        },
        token::{token_reader::TokenDataTriple, CustomERC20, HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::Delta,
    types::{Address, StoreReader},
};

impl MarketCounts for Dynamic {
    fn process<'a>(
        market_counts: &MarketCountsTuple,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        let counts = Self::get_leg(market_counts);

        // Dynamic with hardcoded ERC20 (3)
        for _ in 0..counts.inner[0] {
            process_market::<Dynamic, Pair<ETH, HardcodedERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..counts.inner[1] {
            process_market::<Dynamic, Pair<HardcodedERC20, ETH>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..counts.inner[2] {
            process_market::<Dynamic, Pair<HardcodedERC20, HardcodedERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        // Dynamic with custom ERC20 (3)
        for _ in 0..counts.inner[3] {
            process_market::<Dynamic, Pair<ETH, CustomERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..counts.inner[4] {
            process_market::<Dynamic, Pair<CustomERC20, ETH>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..counts.inner[5] {
            process_market::<Dynamic, Pair<CustomERC20, CustomERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        // Dynamic with mixture of hardcoded and custom ERC20 (2)
        for _ in 0..counts.inner[6] {
            process_market::<Dynamic, Pair<HardcodedERC20, CustomERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        for _ in 0..counts.inner[7] {
            process_market::<Dynamic, Pair<CustomERC20, HardcodedERC20>>(
                msg_sender,
                ctx,
                token_data_triple,
                delta,
            )?;
        }

        Ok(())
    }
}
