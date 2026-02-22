use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::compact_coordinates::CompactCoordinates,
    state::{
        inner_bitmap::preimage::InnerBitmapPreimage, resting_order::RestingOrder, Preimage, SlotKey,
    },
};

#[derive(Clone, Copy)]
pub struct RestingOrderPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q, In>>,
    pub compact_coordinates: CompactCoordinates<In>,
}

impl<M, B, Q, In> Preimage for RestingOrderPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    const SLOT_DISCRIMINATOR: u8 = 7;
    type SlotState = RestingOrder<M, B, Q>;
}
