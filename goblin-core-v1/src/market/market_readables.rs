use crate::{
    axis_helpers::MarketSpec,
    market::CommonMarket,
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<MS: MarketSpec> {
    pub market: CommonMarket<MS>,
    pub market_key: SlotKey<MarketPreimage<MS>>,
}
