use crate::{
    market::{CommonMarket, MarketVariant},
    state::{MarketState, SlotKey},
    token::TokenMarker,
};

pub struct MarketAndKey<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub key: SlotKey<MarketState<M, B, Q>>, // pub key: M::MarketKey<B, Q>,
}
