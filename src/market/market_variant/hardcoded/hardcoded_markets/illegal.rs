///! Stub implementations for illegal combinations of MarketVariant and TokenMarker
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use crate::{
    market::{Hardcoded, HardcodedMarketIndex, HardcodedMarkets, MarketAndKey},
    token::{CustomERC20, HardcodedERC20, ETH},
};

impl HardcodedMarkets<ETH, CustomERC20> for HardcodedMarketIndex<ETH, CustomERC20> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, ETH, CustomERC20>] = &[];
}

impl HardcodedMarkets<CustomERC20, ETH> for HardcodedMarketIndex<CustomERC20, ETH> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, CustomERC20, ETH>] = &[];
}

impl HardcodedMarkets<CustomERC20, CustomERC20> for HardcodedMarketIndex<CustomERC20, CustomERC20> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, CustomERC20, CustomERC20>] = &[];
}

impl HardcodedMarkets<HardcodedERC20, CustomERC20>
    for HardcodedMarketIndex<HardcodedERC20, CustomERC20>
{
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, HardcodedERC20, CustomERC20>] = &[];
}

impl HardcodedMarkets<CustomERC20, HardcodedERC20>
    for HardcodedMarketIndex<CustomERC20, HardcodedERC20>
{
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, CustomERC20, HardcodedERC20>] = &[];
}
