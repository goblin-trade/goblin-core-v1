use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::coordinate::coordinate_item::CoordinateItem,
        bitmap::{
            inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
            FullCoordinates,
        },
    },
    state::{
        bitmap::inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
        resting_order::preimage::RestingOrderPreimage,
        Preimage, SlotKey,
    },
};

pub struct InnerBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q, In>>,
    pub inner_bitmap: InnerBitmap<M, B, Q>,
}

impl<M, B, Q, In> InnerBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn next_item(&self, inner_pos: InnerPos<In>) -> Option<CoordinateItem<M, B, Q, In>> {
        if self.inner_bitmap.active(inner_pos.into()) {
            let preimage = RestingOrderPreimage {
                inner_bitmap_key: self.inner_bitmap_key,
                inner_pos,
            };
            let hash = preimage.hash();
            let resting_order = hash.load();

            return Some(CoordinateItem {
                full_coordinates: FullCoordinates {
                    outer_bitmap_index: self.outer_bitmap_index,
                    outer_pos: self.outer_pos,
                    inner_pos,
                },
                inner_bitmap_key: self.inner_bitmap_key,
                hash,
                resting_order,
            });
        }

        None
    }
}
