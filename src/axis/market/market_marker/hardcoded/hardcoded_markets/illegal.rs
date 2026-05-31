use crate::axis::{
    market::{
        market_marker::hardcoded::hardcoded_market_index::HardcodedMarketIndex, Hardcoded,
        MarketReadables,
    },
    token::{CustomERC20, HardcodedERC20, ETH},
};

///! Stub implementations for illegal combinations of MarketVariant and TokenMarker
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use super::HardcodedMarkets;

impl HardcodedMarkets<ETH, CustomERC20> for HardcodedMarketIndex<ETH, CustomERC20> {
    const HARDCODED_MARKETS: &'static [MarketReadables<Hardcoded, ETH, CustomERC20>] = &[];
}

impl HardcodedMarkets<CustomERC20, ETH> for HardcodedMarketIndex<CustomERC20, ETH> {
    const HARDCODED_MARKETS: &'static [MarketReadables<Hardcoded, CustomERC20, ETH>] = &[];
}

impl HardcodedMarkets<CustomERC20, CustomERC20> for HardcodedMarketIndex<CustomERC20, CustomERC20> {
    const HARDCODED_MARKETS: &'static [MarketReadables<Hardcoded, CustomERC20, CustomERC20>] = &[];
}

impl HardcodedMarkets<HardcodedERC20, CustomERC20>
    for HardcodedMarketIndex<HardcodedERC20, CustomERC20>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<Hardcoded, HardcodedERC20, CustomERC20>] = &[];
}

impl HardcodedMarkets<CustomERC20, HardcodedERC20>
    for HardcodedMarketIndex<CustomERC20, HardcodedERC20>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<Hardcoded, CustomERC20, HardcodedERC20>] = &[];
}
