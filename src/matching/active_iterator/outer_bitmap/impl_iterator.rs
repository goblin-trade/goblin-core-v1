use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::outer_bitmap::{
            outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator,
        },
        bitmap::range::CustomRange,
    },
};

impl<'a, M, B, Q, In> Iterator for ActiveOuterBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = OuterBitmapItem<M, B, Q, In>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(outer_bitmap_index) = self.linear_iterator.next() {
            let result = self.item.get_outer_bitmap_item(CustomRange {
                start: outer_bitmap_index,
                end: self.limit,
            });

            if result.is_some() {
                return result;
            }
        }

        None
    }
}
