use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::active_outer_bitmap_iterator::ActiveOuterBitmapIterator,
        bitmap::{outer_bitmap_index::OuterBitmapIndex, Coordinate},
    },
    state::{
        outer_bitmap::{
            active_outer_bitmap::ActiveOuterBitmap, outer_bitmap_state::OuterBitmapState,
            preimage::OuterBitmapPreimage,
        },
        Preimage,
    },
};

impl<'a, M, B, Q, In> Iterator for ActiveOuterBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    type Item = (OuterBitmapIndex<In>, ActiveOuterBitmap<M, B, Q>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(outer_bitmap_index) = self.outer_bitmap_index {
            let mut linear_iterator = outer_bitmap_index.iter();

            while let Some(outer_bitmap_index) = linear_iterator.next() {
                let preimage = OuterBitmapPreimage {
                    market_key: *self.market_key,
                    outer_bitmap_index,
                };
                let hash = preimage.hash();
                let outer_bitmap = hash.load();
                let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

                if let OuterBitmapState::Active(active_outer_bitmap) = outer_bitmap_state {
                    // Move the cursor and return the current value
                    self.outer_bitmap_index = linear_iterator.next();
                    return Some((outer_bitmap_index, active_outer_bitmap));
                } else if outer_bitmap_index == self.limit {
                    return None;
                }
            }

            None
        } else {
            None
        }
    }
}
