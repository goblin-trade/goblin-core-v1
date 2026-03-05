use super::HardcodedMarkets;
use crate::{
    axis::{
        leg::Pair,
        market::{
            market_marker::hardcoded::hardcoded_market_index::HardcodedMarketIndex, CommonMarket,
            Hardcoded, MarketAndKey,
        },
        token::{
            token_marker::hardcoded_erc20::hardcoded_erc20_index::HardcodedERC20Index,
            HardcodedERC20, ETH,
        },
    },
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    types::Tuple,
};

impl HardcodedMarkets<ETH, HardcodedERC20> for HardcodedMarketIndex<ETH, HardcodedERC20> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, ETH, HardcodedERC20>] =
        &[MarketAndKey {
            market: CommonMarket::new(
                Pair::new((), HardcodedERC20Index(0)),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(100),
                    QuoteLotsPerQuoteUnit::new(1000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarkets<HardcodedERC20, ETH> for HardcodedMarketIndex<HardcodedERC20, ETH> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, HardcodedERC20, ETH>] =
        &[MarketAndKey {
            market: CommonMarket::new(
                Pair::new(HardcodedERC20Index(1), ()),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(200),
                    QuoteLotsPerQuoteUnit::new(2000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarkets<HardcodedERC20, HardcodedERC20>
    for HardcodedMarketIndex<HardcodedERC20, HardcodedERC20>
{
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, HardcodedERC20, HardcodedERC20>] =
        &[MarketAndKey {
            market: CommonMarket::new(
                Pair::new(HardcodedERC20Index(0), HardcodedERC20Index(1)),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(300),
                    QuoteLotsPerQuoteUnit::new(3000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}
