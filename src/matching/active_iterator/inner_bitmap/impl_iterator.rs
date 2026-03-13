use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::inner_bitmap::{
            inner_bitmap_item::InnerBitmapItem, ActiveInnerBitmapIterator,
        },
        bitmap::range::Range,
    },
};

impl<'a, M, B, Q, In> Iterator for ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = InnerBitmapItem<M, B, Q, In>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try advancing current outer_pos iterator
            while let Some(outer_pos) = self.linear_iterator.next() {
                let result = self.item.get_inner_bitmap_item(Range {
                    start: outer_pos,
                    limit: self.limit,
                });

                if result.is_some() {
                    return result;
                }
            }

            // Try to load the next outer bitmap if no active OuterPos was found
            // in the current one. Reset OuterPos to start position.
            self.item = self.active_outer_bitmap_iterator.next()?;

            // TODO get rid of on_limit and evaluate directly?
            // This way on_limit is checked only when we move to a new OuterBitmapIndex,
            // not on every call
            let limit = self.limit.get_limit(
                self.item.outer_bitmap_index,
                self.active_outer_bitmap_iterator.limit,
            );

            self.linear_iterator = In::outer_pos_iter(Range {
                start: In::start_value(),
                limit,
            });
        }
    }
}
