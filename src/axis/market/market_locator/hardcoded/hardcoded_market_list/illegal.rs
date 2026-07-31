use crate::axis::{
    leg::Pair,
    market::{Hardcoded, MarketReadables},
    token::{CustomERC20, HardcodedERC20, ETH},
};

///! Stub implementations for illegal combinations of MarketVariant and TokenMarkerInner
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use super::HardcodedMarketList;

impl HardcodedMarketList for Pair<ETH, CustomERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(Hardcoded, Pair<ETH, CustomERC20>)>] =
        &[];
}

impl HardcodedMarketList for Pair<CustomERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(Hardcoded, Pair<CustomERC20, ETH>)>] =
        &[];
}

impl HardcodedMarketList for Pair<CustomERC20, CustomERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(
        Hardcoded,
        Pair<CustomERC20, CustomERC20>,
    )>] = &[];
}

impl HardcodedMarketList for Pair<HardcodedERC20, CustomERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(
        Hardcoded,
        Pair<HardcodedERC20, CustomERC20>,
    )>] = &[];
}

impl HardcodedMarketList for Pair<CustomERC20, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(
        Hardcoded,
        Pair<CustomERC20, HardcodedERC20>,
    )>] = &[];
}
