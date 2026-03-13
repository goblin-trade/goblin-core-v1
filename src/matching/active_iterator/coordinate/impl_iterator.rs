use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::coordinate::{coordinate_item::CoordinateItem, CoordinateIterator},
        bitmap::range::Range,
    },
};

impl<'a, M, B, Q, In> Iterator for CoordinateIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = CoordinateItem<M, B, Q, In>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(coordinates) = self.linear_iterator.next() {
                let result = self.item.get_resting_order_item(Range {
                    start: coordinates,
                    limit: self.limit,
                });

                if result.is_some() {
                    return result;
                }
            }

            // Try to load the next inner bitmap if no active coordinate was found
            // in the current one. Reset coordinate iterator to start position.
            self.item = self.active_inner_bitmap_iterator.next()?;

            let limit = self.limit.get_limit(
                self.item.outer_bitmap_index
                    == self
                        .active_inner_bitmap_iterator
                        .active_outer_bitmap_iterator
                        .limit
                    && self.item.outer_pos == self.active_inner_bitmap_iterator.limit,
            );

            self.linear_iterator = In::inner_pos_iter(Range {
                start: In::start_value(),
                limit,
            });
        }
    }
}
