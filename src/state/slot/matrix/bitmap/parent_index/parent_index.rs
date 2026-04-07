use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    quantities::OuterBitmapIndexV2,
    state::{bitmap::outer_bitmap::preimage::OuterBitmapPreimage, MarketPreimage, SlotKey},
};

pub trait ParentIndex<M, B, Q>: Clone + Copy + PartialEq + PartialOrd
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    type SlotKey;
}

impl<M, B, Q> ParentIndex<M, B, Q> for ()
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    type SlotKey = SlotKey<MarketPreimage<M, B, Q>>;
}

impl<M, B, Q> ParentIndex<M, B, Q> for OuterBitmapIndexV2
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    type SlotKey = SlotKey<OuterBitmapPreimage<M, B, Q>>;
}
