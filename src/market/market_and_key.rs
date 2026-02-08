use crate::{
    market::{CommonMarket, MarketMarker},
    state::{MarketPreimage, SlotKey},
    token::TokenMarker,
};

pub struct MarketAndKey<M: MarketMarker, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub key: SlotKey<MarketPreimage<M, B, Q>>,
}
