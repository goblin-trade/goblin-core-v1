use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
    state::{outer_bitmap::OuterBitmap, MarketPreimage, Preimage, SlotKey},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OuterBitmapPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub outer_bitmap_index: OuterBitmapIndex<In>,
}

impl<M, B, Q, In> Preimage for OuterBitmapPreimage<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    const SLOT_DISCRIMINATOR: u8 = 5;
    type SlotState = OuterBitmap<M, B, Q>;
}
