use crate::{
    axis::market::{market_spec::MarketSpec, CommonMarket},
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<MS: MarketSpec> {
    pub market: CommonMarket<MS>,
    pub market_key: SlotKey<MarketPreimage<MS>>,
}
