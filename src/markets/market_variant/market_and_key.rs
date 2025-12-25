use crate::{
    markets::{CommonMarket, MarketVariant},
    token::TokenMarker,
};

pub struct MarketAndKey<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub market: CommonMarket<M, B, Q>,
    pub key: M::MarketKey<B, Q>,
}
