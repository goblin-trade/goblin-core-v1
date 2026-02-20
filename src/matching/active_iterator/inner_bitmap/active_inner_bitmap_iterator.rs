use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::outer_bitmap::{
            outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator,
        },
        bitmap::outer_pos::OuterPos,
    },
};

pub struct ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub active_outer_bitmap_iterator: ActiveOuterBitmapIterator<'a, M, B, Q, In>,

    /// The last returned outer bitmap item
    pub outer_bitmap_item: Option<OuterBitmapItem<M, B, Q, In>>,

    /// Begin lookup from this position
    pub outer_pos: Option<OuterPos<In>>,
}

impl<'a, M, B, Q, In> ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn new(
        active_outer_bitmap_iterator: ActiveOuterBitmapIterator<'a, M, B, Q, In>,
        outer_pos: OuterPos<In>,
    ) -> Self {
        Self {
            active_outer_bitmap_iterator,
            outer_bitmap_item: None,
            outer_pos: Some(outer_pos),
        }
    }
}
