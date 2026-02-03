use crate::{
    market::{CommonMarket, HardcodedMarketIndex, Hardcoded, HardcodedMarketList, MarketAndKey},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    token::{HardcodedERC20, HardcodedERC20Index, ETH},
    types::{Pair, Tuple},
};

impl HardcodedMarketList<ETH, HardcodedERC20> for HardcodedMarketIndex<ETH, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, ETH, HardcodedERC20>] =
        &[MarketAndKey {
            market: CommonMarket::new(
                Pair::new((), HardcodedERC20Index(0)),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(100),
                    QuoteLotsPerQuoteUnit::new(1000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarketList<HardcodedERC20, ETH> for HardcodedMarketIndex<HardcodedERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, HardcodedERC20, ETH>] =
        &[MarketAndKey {
            market: CommonMarket::new(
                Pair::new(HardcodedERC20Index(1), ()),
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
    for HardcodedMarketIndex<HardcodedERC20, HardcodedERC20>
{
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<
        Hardcoded,
        HardcodedERC20,
        HardcodedERC20,
    >] = &[MarketAndKey {
        market: CommonMarket::new(
            Pair::new(HardcodedERC20Index(0), HardcodedERC20Index(1)),
            Tuple::new(
                BaseLotsPerBaseUnit::new(300),
                QuoteLotsPerQuoteUnit::new(3000),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ),
        key: SlotKey::new([0u8; 32]),
    }];
}
