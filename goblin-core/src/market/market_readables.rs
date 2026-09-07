use crate::{
    axis_helpers::TokenPair,
    market::CommonMarket,
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<TP: TokenPair> {
    pub market: CommonMarket<TP>,
    pub market_key: SlotKey<MarketPreimage<TP>>,
}
