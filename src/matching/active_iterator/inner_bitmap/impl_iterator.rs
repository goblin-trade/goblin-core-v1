use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::active_iterator::inner_bitmap::{
        inner_bitmap_item::InnerBitmapItem, ActiveInnerBitmapIterator,
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
                let result = self.item.get_inner_bitmap_item(outer_pos);

                if result.is_some() {
                    return result;
                }
            }

            // Try to load the next outer bitmap if no active OuterPos was found
            // in the current one. Reset OuterPos to start position.
            self.item = self.active_outer_bitmap_iterator.next()?;
            let limit = self.limit.adjust_limit(self.on_limit());

            self.linear_iterator = In::outer_pos_iter(In::start_value()..=limit);
        }
    }
}
