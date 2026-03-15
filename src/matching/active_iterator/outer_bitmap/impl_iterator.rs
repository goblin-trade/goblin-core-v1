use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::active_iterator::outer_bitmap::{
        outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator,
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
        let outer_bitmap_index = self.linear_iterator.next()?;
        self.inner_item.next_item(outer_bitmap_index)

        // No reset step here unlike other iterators
        // Symmetric version
        // loop {
        //     if let Some(outer_bitmap_index) = self.linear_iterator.next() {
        //         let result = self.inner_item.next_item(outer_bitmap_index);
        //         if result.is_some() {
        //             return result;
        //         }
        //     } else {
        //         return None;
        //     }
        // }
    }
}
