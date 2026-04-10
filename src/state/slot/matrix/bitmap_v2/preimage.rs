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
pub struct BitmapPreimageV2<M, B, Q, const BIT_OFFSET: usize, const BIT_COUNT: usize>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    BitmapIndexV2<BIT_OFFSET, BIT_COUNT>: OrderedIndex,
    <BitmapIndexV2<BIT_OFFSET, BIT_COUNT> as OrderedIndex>::Prev: OrderedIndex,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub outer_index: OuterIndex<BitmapIndexV2<BIT_OFFSET, BIT_COUNT>>,
}

impl<M, B, Q, const BIT_OFFSET: usize, const BIT_COUNT: usize> Preimage
    for BitmapPreimageV2<M, B, Q, BIT_OFFSET, BIT_COUNT>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    BitmapIndexV2<BIT_OFFSET, BIT_COUNT>: OrderedIndex,
    <BitmapIndexV2<BIT_OFFSET, BIT_COUNT> as OrderedIndex>::Prev: OrderedIndex,
{
    // Discriminator 5 for both bitmaps
    // TODO update RestingOrderBitmap with discriminator 6
    const SLOT_DISCRIMINATOR: u8 = 5;
    type SlotState = BitmapV2<BIT_OFFSET, BIT_COUNT>;
}
