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
            if let Some(outer_pos) = self.outer_pos {
                let mut linear_iterator = outer_pos.iter();
                while let Some(outer_pos) = linear_iterator.next() {
                    if self.outer_bitmap_item.active_outer_bitmap.active(outer_pos) {
                        let preimage = InnerBitmapPreimage {
                            outer_bitmap_key: self.outer_bitmap_item.outer_bitmap_key,
                            outer_pos,
                        };
                        let hash = preimage.hash();
                        let inner_bitmap = hash.load();

                        self.outer_pos = linear_iterator.next();
                        return Some(InnerBitmapItem {
                            outer_bitmap_index: self.outer_bitmap_item.outer_bitmap_index,
                            outer_pos,
                            inner_bitmap,
                        });
                    }
                }
            } else {
                // if outer_pos is None try to load the next outer bitmap
                if let Some(item) = self.active_outer_bitmap_iterator.next() {
                    self.outer_bitmap_item = item;
                    self.outer_pos = Some(In::start_value());
                } else {
                    return None;
                }
            }
        }
    }
}
