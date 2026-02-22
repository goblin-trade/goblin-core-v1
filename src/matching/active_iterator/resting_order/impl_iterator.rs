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
                let mut row_linear_iterator = coordinates.row.iter();
                while let Some(row) = row_linear_iterator.next() {
                    if self.inner_bitmap_item.limit_reached && self.limit.row.closer_to_centre(row)
                    {
                        return None;
                    }
                    let limit_reached =
                        self.inner_bitmap_item.limit_reached && self.limit.row == row;

                    let mut column_linear_iterator = coordinates.column.iter();
                    while let Some(column) = column_linear_iterator.next() {
                        let cursor = InnerCoordinates { row, column };
                        if self.inner_bitmap_item.inner_bitmap.active_v2(cursor) {
                            let preimage = RestingOrderPreimage {
                                inner_bitmap_key: self.inner_bitmap_item.inner_bitmap_key,
                                compact_coordinates: CompactCoordinates::from(cursor),
                            };
                            let resting_order_key = preimage.hash();
                            let resting_order = resting_order_key.load();

                            // TODO update self.inner_coordinates
                            // Could could be simplified. A single .next() defined on CompactCoordinates
                            return Some(RestingOrderItem {
                                outer_bitmap_index: self.inner_bitmap_item.outer_bitmap_index,
                                outer_pos: self.inner_bitmap_item.outer_pos,
                                inner_coordinates: cursor,
                                resting_order_key,
                                resting_order,
                                limit_reached,
                            });
                        }
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
