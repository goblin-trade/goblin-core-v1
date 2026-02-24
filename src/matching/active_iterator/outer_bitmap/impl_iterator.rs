use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::outer_bitmap::{
            outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator,
        },
        bitmap::Coordinate,
    },
    state::{
        outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
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
    type Item = OuterBitmapItem<M, B, Q, In>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(outer_bitmap_index) = self.linear_iterator.next() {
            if self.limit.closer_to_centre(outer_bitmap_index) {
                return None;
            }
            let limit_reached = self.limit == outer_bitmap_index;

            let preimage = OuterBitmapPreimage {
                market_key: *self.market_key,
                outer_bitmap_index,
            };
            let outer_bitmap_key = preimage.hash();
            let outer_bitmap = outer_bitmap_key.load();
            let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

            if let OuterBitmapState::Active(active_outer_bitmap) = outer_bitmap_state {
                // Move the cursor and return the current value
                self.linear_iterator.next();
                return Some(OuterBitmapItem {
                    outer_bitmap_index,
                    outer_bitmap_key,
                    active_outer_bitmap,
                    on_limit: limit_reached,
                });
            }
        }

        None
    }
}
