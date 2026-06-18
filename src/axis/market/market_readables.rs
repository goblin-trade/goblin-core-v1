use crate::{
    axis::{
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_reader::TokenReader,
    },
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<M: MarketMarker, B: TokenReader, Q: TokenReader> {
    pub market: CommonMarket<M, B, Q>,
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
}
