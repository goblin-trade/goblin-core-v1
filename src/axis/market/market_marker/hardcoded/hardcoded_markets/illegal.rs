use crate::axis::{
    leg::Pair,
    market::{
        market_marker::hardcoded::hardcoded_market_index::HardcodedMarketIndex, Hardcoded,
        MarketReadables,
    },
    token::{CustomERC20, HardcodedERC20, ETH},
};

///! Stub implementations for illegal combinations of MarketVariant and TokenMarkerInner
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use super::HardcodedMarkets;

impl HardcodedMarkets<Pair<ETH, CustomERC20>> for HardcodedMarketIndex<Pair<ETH, CustomERC20>> {
    const HARDCODED_MARKETS: &'static [MarketReadables<(Hardcoded, Pair<ETH, CustomERC20>)>] = &[];
}

impl HardcodedMarkets<Pair<CustomERC20, ETH>> for HardcodedMarketIndex<Pair<CustomERC20, ETH>> {
    const HARDCODED_MARKETS: &'static [MarketReadables<(Hardcoded, Pair<CustomERC20, ETH>)>] = &[];
}

impl HardcodedMarkets<Pair<CustomERC20, CustomERC20>>
    for HardcodedMarketIndex<Pair<CustomERC20, CustomERC20>>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<(
        Hardcoded,
        Pair<CustomERC20, CustomERC20>,
    )>] = &[];
}

impl HardcodedMarkets<Pair<HardcodedERC20, CustomERC20>>
    for HardcodedMarketIndex<Pair<HardcodedERC20, CustomERC20>>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<(
        Hardcoded,
        Pair<HardcodedERC20, CustomERC20>,
    )>] = &[];
}

impl HardcodedMarkets<Pair<CustomERC20, HardcodedERC20>>
    for HardcodedMarketIndex<Pair<CustomERC20, HardcodedERC20>>
{
    const HARDCODED_MARKETS: &'static [MarketReadables<(
        Hardcoded,
        Pair<CustomERC20, HardcodedERC20>,
    )>] = &[];
}
