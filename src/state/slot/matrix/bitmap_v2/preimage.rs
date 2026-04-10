use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    state::{
        bitmap_v2::{
            bitmap_index_v2::BitmapIndexV2, ordered_index::OrderedIndex, outer_index::OuterIndex,
            BitmapV2,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitmapPreimageV2<M, B, Q, I>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    I: OrderedIndex,
    I::Prev: OrderedIndex,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub outer_index: OuterIndex<I>,
}

impl<M, B, Q, I> Preimage for BitmapPreimageV2<M, B, Q, I>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    I: BitmapIndexV2,
    I::Prev: OrderedIndex,
{
    // Discriminator 5 for both bitmaps
    // TODO update RestingOrderBitmap with discriminator 6
    const SLOT_DISCRIMINATOR: u8 = 5;
    type SlotState = BitmapV2<I>;
}
