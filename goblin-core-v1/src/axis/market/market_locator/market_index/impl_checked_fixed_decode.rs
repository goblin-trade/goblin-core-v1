use crate::{
    axis::market::{
        market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
        market_spec::MarketSpec,
    },
    input_processor::CheckedFixedDecode,
};

impl<'a, MS> CheckedFixedDecode<'a> for MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    const MAX: Self = Self::new(<MS::Pair as HardcodedMarketList>::HARDCODED_MARKET_LIST.len() - 1);
}
