use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::{
            inner_bitmap::{inner_bitmap_item::InnerBitmapItem, ActiveInnerBitmapIterator},
            outer_bitmap::{outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator},
        },
        bitmap::{outer_pos::OuterPos, Coordinate},
    },
    state::{
        inner_bitmap::preimage::InnerBitmapPreimage,
        outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
        Preimage,
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
            if let Some(outer_item) = self.outer_bitmap_item {
                if let Some(outer_pos) = self.outer_pos {
                    let mut linear_iterator = outer_pos.iter();

                    while let Some(outer_pos) = linear_iterator.next() {
                        if outer_item.active_outer_bitmap.active(outer_pos) {
                            let preimage = InnerBitmapPreimage {
                                outer_bitmap_key: outer_item.outer_bitmap_key,
                                outer_pos,
                            };
                            let inner_bitmap_key = preimage.hash();
                            let inner_bitmap = inner_bitmap_key.load();

                            self.outer_pos = linear_iterator.next();
                            return Some(InnerBitmapItem {
                                outer_bitmap_index: outer_item.outer_bitmap_index,
                                outer_pos,
                                inner_bitmap,
                            });
                        }
                    }
                    self.outer_pos = None;
                } else {
                    // TODO depending on side, default is different
                    self.outer_pos = Some(OuterPos::new(0));
                }
            } else {
                if let Some(outer_bitmap_item) = self.active_outer_bitmap_iterator.next() {
                    self.outer_bitmap_item = Some(outer_bitmap_item);
                } else {
                    return None;
                }
            }
        }
    }
}
