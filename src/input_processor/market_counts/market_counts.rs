use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    market::{process_market, Dynamic, Hardcoded},
    settlement::Delta,
    token::{CustomToken, ERC20, ETH},
};

/// The number of markets of each type
pub struct MarketCounts {
    inner: [u8; 6],
}

impl MarketCounts {
    pub fn new(inner: [u8; 6]) -> Self {
        Self { inner }
    }

    /// Process legal combinations of market types
    pub fn process_markets(
        &self,
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError> {
        // Hardcoded
        for _ in 0..self.inner[0] {
            process_market::<Hardcoded, ETH, ERC20>(ctx, offset, len, delta, custom_erc20_list)?;
        }

        for _ in 0..self.inner[1] {
            process_market::<Hardcoded, ERC20, ETH>(ctx, offset, len, delta, custom_erc20_list)?;
        }

        for _ in 0..self.inner[2] {
            process_market::<Hardcoded, ERC20, ERC20>(ctx, offset, len, delta, custom_erc20_list)?;
        }

        // Dynamic
        for _ in 0..self.inner[3] {
            process_market::<Dynamic, ETH, ERC20>(ctx, offset, len, delta, custom_erc20_list)?;
        }
        for _ in 0..self.inner[4] {
            process_market::<Dynamic, ERC20, ETH>(ctx, offset, len, delta, custom_erc20_list)?;
        }
        for _ in 0..self.inner[5] {
            process_market::<Dynamic, ERC20, ERC20>(ctx, offset, len, delta, custom_erc20_list)?;
        }

        Ok(())
    }
}
