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
            if let Some(outer_pos) = self.linear_iterator.next() {
                let result = self.inner_item.next_item(outer_pos);
                if result.is_some() {
                    return result;
                }
            } else {
                // Load the next item and reset the linear iterator
                self.inner_item = self.inner_iterator.next()?;
                let end = self.limit.adjust_limit(self.on_limit());
                self.linear_iterator = In::outer_pos_iter(In::start_value()..=end);
            }
        }
    }
}
