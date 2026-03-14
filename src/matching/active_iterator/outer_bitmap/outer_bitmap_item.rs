use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::inner_bitmap::inner_bitmap_item::InnerBitmapItem,
        bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, range::Range},
    },
    state::{
        inner_bitmap::preimage::InnerBitmapPreimage,
        outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, preimage::OuterBitmapPreimage},
        Preimage, SlotKey,
    },
};

pub struct OuterBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// Index of the active outer bitmap
    pub outer_bitmap_index: OuterBitmapIndex<In>,

    /// The slot key
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,

    /// The active outer bitmap read from slot
    pub active_outer_bitmap: ActiveOuterBitmap<M, B, Q>,
}

impl<M, B, Q, In> OuterBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn get_inner_bitmap_item(
        &self,
        outer_pos_range: Range<OuterPos<In>>,
    ) -> Option<InnerBitmapItem<M, B, Q, In>> {
        if self.active_outer_bitmap.active(outer_pos_range.start) {
            let preimage = InnerBitmapPreimage {
                outer_bitmap_key: self.outer_bitmap_key,
                outer_pos: outer_pos_range.start,
            };
            let inner_bitmap_key = preimage.hash();
            let inner_bitmap = inner_bitmap_key.load();

            return Some(InnerBitmapItem {
                outer_bitmap_index: self.outer_bitmap_index,
                outer_pos: outer_pos_range.start,
                inner_bitmap_key,
                inner_bitmap,
            });
        }

        None
    }
}
