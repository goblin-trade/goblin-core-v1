use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::outer_bitmap_index::OuterBitmapIndex,
        generic_iterator::{outer_bitmap_index::outer_bitmap_item::OuterBitmapItemV2, InnerItem},
    },
    state::{
        bitmap::outer_bitmap::{
            outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};

#[derive(Clone, Copy)]
pub struct MarketItemV2<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Market key
    pub market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
}

impl<'a, M, B, Q> Iterator for MarketItemV2<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<'a, M, B, Q, In> InnerItem<OuterBitmapIndex<In>, OuterBitmapItemV2<M, B, Q, In>>
    for MarketItemV2<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    fn next_item(&self, pos: OuterBitmapIndex<In>) -> Option<OuterBitmapItemV2<M, B, Q, In>> {
        let preimage = OuterBitmapPreimage {
            market_key: *self.market_key,
            outer_bitmap_index: pos,
        };
        let outer_bitmap_key = preimage.hash();
        let outer_bitmap = outer_bitmap_key.load();
        let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

        if let OuterBitmapState::Active(active_outer_bitmap) = outer_bitmap_state {
            return Some(OuterBitmapItemV2 {
                outer_bitmap_index: pos,
                outer_bitmap_key,
                active_outer_bitmap,
            });
        }

        None
    }
}
