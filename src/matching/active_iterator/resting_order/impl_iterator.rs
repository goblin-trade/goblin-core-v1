use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::resting_order::{
            resting_order_item::RestingOrderItem, RestingOrderIterator,
        },
        bitmap::{
            compact_coordinates::CompactCoordinates, inner_coordinates::InnerCoordinates,
            Coordinate,
        },
    },
    state::{resting_order::preimage::RestingOrderPreimage, Preimage},
};

impl<'a, M, B, Q, In> Iterator for RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = RestingOrderItem<M, B, Q, In>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(coordinates) = self.coordinates_iter.next() {
                let inner_coordinates = InnerCoordinates::from(coordinates);

                if self.inner_bitmap_item.limit_reached
                    && self.limit.closer_to_centre(inner_coordinates.row)
                {
                    return None;
                }
                let limit_reached =
                    self.inner_bitmap_item.limit_reached && self.limit == inner_coordinates.row;

                if self
                    .inner_bitmap_item
                    .inner_bitmap
                    .active(inner_coordinates)
                {
                    let preimage = RestingOrderPreimage {
                        inner_bitmap_key: self.inner_bitmap_item.inner_bitmap_key,
                        compact_coordinates: CompactCoordinates::from(coordinates),
                    };
                    let resting_order_key = preimage.hash();
                    let resting_order = resting_order_key.load();

                    self.coordinates_iter.next();

                    return Some(RestingOrderItem {
                        outer_bitmap_index: self.inner_bitmap_item.outer_bitmap_index,
                        outer_pos: self.inner_bitmap_item.outer_pos,
                        inner_coordinates,
                        resting_order_key,
                        resting_order,
                        limit_reached,
                    });
                }
            }

            if let Some(item) = self.active_inner_bitmap_iterator.next() {
                // Go to the next inner bitmap
                // Reset coordinates to start position
                self.inner_bitmap_item = item;
                self.coordinates_iter = In::coordinates_iter(In::start_value());
            } else {
                return None;
            }
        }
    }
}
