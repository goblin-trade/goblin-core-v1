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
            if let Some(coordinates) = self.coordinates {
                let mut linear_iterator = coordinates.iter();
                while let Some(coordinates) = linear_iterator.next() {
                    if self.inner_bitmap_item.limit_reached
                        && self.limit.row.closer_to_centre(coordinates.row)
                    {
                        return None;
                    }
                    let limit_reached =
                        self.inner_bitmap_item.limit_reached && self.limit.row == coordinates.row;

                    if self.inner_bitmap_item.inner_bitmap.active_v2(coordinates) {
                        let preimage = RestingOrderPreimage {
                            inner_bitmap_key: self.inner_bitmap_item.inner_bitmap_key,
                            compact_coordinates: CompactCoordinates::from(coordinates),
                        };
                        let resting_order_key = preimage.hash();
                        let resting_order = resting_order_key.load();

                        self.coordinates = linear_iterator.next();

                        return Some(RestingOrderItem {
                            outer_bitmap_index: self.inner_bitmap_item.outer_bitmap_index,
                            outer_pos: self.inner_bitmap_item.outer_pos,
                            inner_coordinates: coordinates,
                            resting_order_key,
                            resting_order,
                            limit_reached,
                        });
                    }
                }
            } else {
                // if coordinates is None try to load the next inner bitmap
                if let Some(item) = self.active_inner_bitmap_iterator.next() {
                    self.inner_bitmap_item = item;
                    self.coordinates = Some(InnerCoordinates::start_value());
                } else {
                    return None;
                }
            }
        }
    }
}
