use crate::{
    axis::market::CommonMarket,
    axis_helpers::MarketSpec,
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<MS: MarketSpec> {
    pub market: CommonMarket<MS>,
    pub market_key: SlotKey<MarketPreimage<MS>>,
}
