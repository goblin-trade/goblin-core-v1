use crate::{
    markets::{CommonMarket, MarketVariant},
    token::TokenMarker,
};

pub struct MarketWithKey<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub common_market: CommonMarket<M, B, Q>,
    pub key: M::MarketKey<B, Q>,
}

pub struct MarketWithKeyRef<'a, M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    pub common_market: &'a CommonMarket<M, B, Q>,
    pub key: &'a M::MarketKey<B, Q>,
}
