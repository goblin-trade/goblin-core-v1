use super::HardcodedMarkets;
use crate::{
    axis::{
        leg::Pair,
        market::{CommonMarket, Hardcoded, MarketReadables},
        token::{token_marker::HardcodedERC20Index, ETHStub, HardcodedERC20, ETH},
    },
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    types::Tuple,
};

impl HardcodedMarkets<Pair<ETH, HardcodedERC20>> for Pair<ETH, HardcodedERC20> {
    const HARDCODED_MARKETS: &'static [MarketReadables<(Hardcoded, Pair<ETH, HardcodedERC20>)>] =
        &[MarketReadables {
            market: CommonMarket::new(
                Pair::new(ETHStub, HardcodedERC20Index(0)),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(100),
                    QuoteLotsPerQuoteUnit::new(1000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarkets<Pair<HardcodedERC20, ETH>> for Pair<HardcodedERC20, ETH> {
    const HARDCODED_MARKETS: &'static [MarketReadables<(Hardcoded, Pair<HardcodedERC20, ETH>)>] =
        &[MarketReadables {
            market: CommonMarket::new(
                Pair::new(HardcodedERC20Index(1), ETHStub),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(200),
                    QuoteLotsPerQuoteUnit::new(2000),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarkets<Pair<HardcodedERC20, HardcodedERC20>>
    for Pair<HardcodedERC20, HardcodedERC20>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<(
        Hardcoded,
        Pair<HardcodedERC20, HardcodedERC20>,
    )>] = &[MarketReadables {
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
