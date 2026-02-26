use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::resting_order::{
            resting_order_item::RestingOrderItem, RestingOrderIterator,
        },
        bitmap::range::Range,
    },
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
            if let Some(item) = self.active_inner_bitmap_iterator.next() {
                self.item = item;
                self.linear_iterator = In::coordinates_iter(In::start_value());
            } else {
                return None;
            }
        }
    }
}
