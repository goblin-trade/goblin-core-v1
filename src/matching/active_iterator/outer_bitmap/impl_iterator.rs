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
        self.item.get_outer_bitmap_item(outer_bitmap_index)
    }
}
