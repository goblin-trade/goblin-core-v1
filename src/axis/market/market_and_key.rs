use crate::{
    axis::{
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
    },
    state::{MarketPreimage, SlotKey},
};

pub struct MarketAndKey<M: MarketMarker, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub key: SlotKey<MarketPreimage<M, B, Q>>,
}
