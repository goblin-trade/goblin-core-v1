use crate::{
    markets::{CommonMarket, HardcodedMarket, HardcodedMarketList},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::HardcodedMarketKey,
    token::{HardcodedToken, TokenIndex},
    token::{ERC20, ETH},
    types::Pair,
};

impl HardcodedMarketList<(ETH, ERC20)> for HardcodedMarket<(ETH, ERC20)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ETH, ERC20)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: TokenIndex::<HardcodedToken>::new(0),
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(100),
                quote: QuoteLotsPerQuoteUnit::new(1000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<(ERC20, ETH)> for HardcodedMarket<(ERC20, ETH)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ERC20, ETH)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: TokenIndex::<HardcodedToken>::new(1),
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(200),
                quote: QuoteLotsPerQuoteUnit::new(2000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<(ERC20, ERC20)> for HardcodedMarket<(ERC20, ERC20)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ERC20, ERC20)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: Pair {
                base: TokenIndex::<HardcodedToken>::new(0),
                quote: TokenIndex::<HardcodedToken>::new(1),
            },
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(300),
                quote: QuoteLotsPerQuoteUnit::new(3000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}
