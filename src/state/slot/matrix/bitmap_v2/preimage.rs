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
pub struct BitmapPreimageV2<M, B, Q, const BITS: u16>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    BitmapIndexV2<BITS>: OrderedIndex,
    <BitmapIndexV2<BITS> as OrderedIndex>::Prev: OrderedIndex,
{
    pub market_key: SlotKey<MarketPreimage<M, B, Q>>,
    pub outer_index: OuterIndex<BitmapIndexV2<BITS>>,
}

impl<M, B, Q, const BITS: u16> Preimage for BitmapPreimageV2<M, B, Q, BITS>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    BitmapIndexV2<BITS>: OrderedIndex,
    <BitmapIndexV2<BITS> as OrderedIndex>::Prev: OrderedIndex,
{
    // Discriminator 5 for both bitmaps
    // TODO update RestingOrderBitmap with discriminator 6
    const SLOT_DISCRIMINATOR: u8 = 5;
    type SlotState = BitmapV2<BITS>;
}
