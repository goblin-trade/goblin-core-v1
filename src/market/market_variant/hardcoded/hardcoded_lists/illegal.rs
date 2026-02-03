///! Stub implementations for illegal combinations of MarketVariant and TokenMarker
///! They are never used, but are needed for satisfying the compiler.
///!
///! Hardcoded markets only have HardcodedERC20. They cannot have CustomERC20.
use crate::{
    market::{DangerousMarketIndex, Hardcoded, HardcodedMarketList, MarketAndKey},
    token::{CustomERC20, HardcodedERC20, ETH},
};

impl HardcodedMarketList<ETH, CustomERC20> for DangerousMarketIndex<ETH, CustomERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, ETH, CustomERC20>] = &[];
}

impl HardcodedMarketList<CustomERC20, ETH> for DangerousMarketIndex<CustomERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, CustomERC20, ETH>] = &[];
}

impl HardcodedMarketList<CustomERC20, CustomERC20>
    for DangerousMarketIndex<CustomERC20, CustomERC20>
{
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, CustomERC20, CustomERC20>] = &[];
}

impl HardcodedMarketList<HardcodedERC20, CustomERC20>
    for DangerousMarketIndex<HardcodedERC20, CustomERC20>
{
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, HardcodedERC20, CustomERC20>] =
        &[];
}

impl HardcodedMarketList<CustomERC20, HardcodedERC20>
    for DangerousMarketIndex<CustomERC20, HardcodedERC20>
{
    const HARDCODED_MARKET_LIST: &'static [MarketAndKey<Hardcoded, CustomERC20, HardcodedERC20>] =
        &[];
}
