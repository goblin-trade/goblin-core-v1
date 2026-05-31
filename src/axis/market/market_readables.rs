use crate::{
    axis::{
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
    },
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<M: MarketMarker, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
}
