use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::outer_pos::OuterPos,
    state::{
        inner_bitmap::InnerBitmap, outer_bitmap::preimage::OuterBitmapPreimage, Preimage, SlotKey,
    },
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InnerBitmapPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,
    outer_pos: OuterPos<In>,
}

impl<M, B, Q, In> Preimage for InnerBitmapPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = InnerBitmap<M, B, Q>;
}
