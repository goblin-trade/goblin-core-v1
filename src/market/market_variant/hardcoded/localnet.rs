use crate::{
    market::{CommonMarket, Hardcoded, HardcodedMarketList, MarketAndKey},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    token::{ERC20Index, ERC20, ETH},
    types::{Pair, Tuple},
};

impl HardcodedMarketList<ETH, ERC20> for MarketAndKey<Hardcoded, ETH, ERC20> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket {
            token_index_pair: Pair::new((), ERC20Index::new(0)),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(100),
                QuoteLotsPerQuoteUnit::new(1000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        key: SlotKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<ERC20, ETH> for MarketAndKey<Hardcoded, ERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket {
            token_index_pair: Pair::new(ERC20Index::new(1), ()),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(200),
                QuoteLotsPerQuoteUnit::new(2000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        key: SlotKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<ERC20, ERC20> for MarketAndKey<Hardcoded, ERC20, ERC20> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket {
            token_index_pair: Pair::new(ERC20Index::new(0), ERC20Index::new(1)),
            lot_size_pair: Tuple::new(
                BaseLotsPerBaseUnit::new(300),
                QuoteLotsPerQuoteUnit::new(3000),
            ),
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        key: SlotKey::new([0u8; 32]),
    }];
}
