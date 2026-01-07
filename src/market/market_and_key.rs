use crate::{
    market::{CommonMarket, MarketVariant},
    state::{MarketPreimage, MarketState, SlotKey},
    token::TokenMarker,
};

pub struct MarketAndKey<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub key: SlotKey<MarketPreimage<M, B, Q>>,
}
