use crate::{
    market::{CommonMarket, Hardcoded, HardcodedMarketList, MarketAndKey},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    token::{ERC20Index, HardcodedERC20, ETH},
    types::{Pair, Tuple},
};

impl HardcodedMarketList<ETH, HardcodedERC20> for MarketAndKey<Hardcoded, ETH, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket::new(
            Pair::new((), ERC20Index::new(0)),
            Tuple::new(
                BaseLotsPerBaseUnit::new(100),
                QuoteLotsPerQuoteUnit::new(1000),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ),
        key: SlotKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<HardcodedERC20, ETH> for MarketAndKey<Hardcoded, HardcodedERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket::new(
            Pair::new(ERC20Index::new(1), ()),
            Tuple::new(
                BaseLotsPerBaseUnit::new(200),
                QuoteLotsPerQuoteUnit::new(2000),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ),
        key: SlotKey::new([0u8; 32]),
    }];
}

impl HardcodedMarketList<HardcodedERC20, HardcodedERC20>
    for MarketAndKey<Hardcoded, HardcodedERC20, HardcodedERC20>
{
    const HARDCODED_MARKET_LIST: &'static [Self] = &[MarketAndKey {
        market: CommonMarket::new(
            Pair::new(ERC20Index::new(0), ERC20Index::new(1)),
            Tuple::new(
                BaseLotsPerBaseUnit::new(300),
                QuoteLotsPerQuoteUnit::new(3000),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ),
        key: SlotKey::new([0u8; 32]),
    }];
}
