use crate::{
    axis::market::{
        market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
        market_spec::MarketSpec,
    },
    input_processor::FixedDecode,
};

impl<'a, MS> FixedDecode<'a> for MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(ctx: &'a crate::input_processor::DecodeCtx) -> Self {
        let market_index_raw = u8::raw_fixed_decode(ctx) as usize;
        Self::new(market_index_raw)
    }
}
