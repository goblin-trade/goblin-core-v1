use crate::{
    markets::{CommonMarket, HardcodedMarket, HardcodedMarketList},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::HardcodedMarketKey,
    token::{HardcodedToken, TokenIndex, ERC20, ETH},
    types::Tuple,
};

impl HardcodedMarketList<(ETH, ERC20)> for HardcodedMarket<(ETH, ERC20)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ETH, ERC20)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: TokenIndex::<HardcodedToken>::new(0),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(100),
                QuoteLotsPerQuoteUnit::new(1000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<(ERC20, ETH)> for HardcodedMarket<(ERC20, ETH)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ERC20, ETH)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: TokenIndex::<HardcodedToken>::new(1),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(200),
                QuoteLotsPerQuoteUnit::new(2000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<(ERC20, ERC20)> for HardcodedMarket<(ERC20, ERC20)> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<(ERC20, ERC20)>] = &[HardcodedMarket {
        common: CommonMarket {
            token_index_pair: Tuple::new(
                TokenIndex::<HardcodedToken>::new(0),
                TokenIndex::<HardcodedToken>::new(1),
            ),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(300),
                QuoteLotsPerQuoteUnit::new(3000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: HardcodedMarketKey::new([0u8; 32]),
    }];
}
