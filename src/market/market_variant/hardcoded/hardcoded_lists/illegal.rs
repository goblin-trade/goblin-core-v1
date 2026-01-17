///! Stub implementations for illegal combinations of MarketVariant and TokenMarker
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use crate::{
    market::{Hardcoded, HardcodedMarketList, MarketAndKey},
    token::{CustomERC20, HardcodedERC20, ETH},
};

impl HardcodedMarketList<ETH, CustomERC20> for MarketAndKey<Hardcoded, ETH, CustomERC20> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[];
}

impl HardcodedMarketList<CustomERC20, ETH> for MarketAndKey<Hardcoded, CustomERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [Self] = &[];
}

impl HardcodedMarketList<CustomERC20, CustomERC20>
    for MarketAndKey<Hardcoded, CustomERC20, CustomERC20>
{
    const HARDCODED_MARKET_LIST: &'static [Self] = &[];
}

impl HardcodedMarketList<HardcodedERC20, CustomERC20>
    for MarketAndKey<Hardcoded, HardcodedERC20, CustomERC20>
{
    const HARDCODED_MARKET_LIST: &'static [Self] = &[];
}

impl HardcodedMarketList<CustomERC20, HardcodedERC20>
    for MarketAndKey<Hardcoded, CustomERC20, HardcodedERC20>
{
    const HARDCODED_MARKET_LIST: &'static [Self] = &[];
}
