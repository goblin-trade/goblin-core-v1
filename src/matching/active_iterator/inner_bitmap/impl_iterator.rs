use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::inner_bitmap::{
            inner_bitmap_item::InnerBitmapItem, ActiveInnerBitmapIterator,
        },
        bitmap::Coordinate,
    },
    state::{inner_bitmap::preimage::InnerBitmapPreimage, Preimage},
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
            while let Some(outer_pos) = self.outer_pos_iter.next() {
                if self.outer_bitmap_item.limit_reached && self.limit.closer_to_centre(outer_pos) {
                    return None;
                }
                let limit_reached = self.outer_bitmap_item.limit_reached && self.limit == outer_pos;

                if self.outer_bitmap_item.active_outer_bitmap.active(outer_pos) {
                    let preimage = InnerBitmapPreimage {
                        outer_bitmap_key: self.outer_bitmap_item.outer_bitmap_key,
                        outer_pos,
                    };
                    let inner_bitmap_key = preimage.hash();
                    let inner_bitmap = inner_bitmap_key.load();

                    self.outer_pos_iter.next();
                    return Some(InnerBitmapItem {
                        outer_bitmap_index: self.outer_bitmap_item.outer_bitmap_index,
                        outer_pos,
                        inner_bitmap_key,
                        inner_bitmap,
                        limit_reached,
                    });
                }
            }

            if let Some(item) = self.active_outer_bitmap_iterator.next() {
                // Go to the next outer bitmap
                // Reset outer_pos to start position
                self.outer_bitmap_item = item;
                self.outer_pos_iter = In::outer_pos_iter(In::start_value());
            } else {
                return None;
            }
        }
    }
}
