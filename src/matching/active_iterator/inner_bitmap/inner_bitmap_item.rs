use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::coordinate::coordinate_item::CoordinateItem,
        bitmap::{
            inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
            range::Range, FullCoordinates,
        },
    },
    state::{
        inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
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
    pub on_limit: bool,
}

impl<M, B, Q, In> InnerBitmapItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn get_resting_order_item(
        &self,
        coordinates_range: Range<InnerPos<In>>,
    ) -> Option<CoordinateItem<M, B, Q, In>> {
        if self.on_limit && In::closer_to_centre(coordinates_range.limit, coordinates_range.start) {
            return None;
        }
        // let limit_reached = self.on_limit && coordinates_range.on_limit();

        if self.inner_bitmap.active(coordinates_range.start.into()) {
            let preimage = RestingOrderPreimage {
                inner_bitmap_key: self.inner_bitmap_key,
                inner_pos: coordinates_range.start,
            };
            let hash = preimage.hash();
            let resting_order = hash.load();

            return Some(CoordinateItem {
                full_coordinates: FullCoordinates {
                    outer_bitmap_index: self.outer_bitmap_index,
                    outer_pos: self.outer_pos,
                    inner_pos: coordinates_range.start,
                },
                inner_bitmap_key: self.inner_bitmap_key,
                hash,
                resting_order,
                // limit_reached,
            });
        }

        None
    }
}
