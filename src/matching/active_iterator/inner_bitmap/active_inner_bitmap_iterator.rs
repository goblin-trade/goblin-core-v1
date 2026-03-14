use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::outer_bitmap::{
            outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator,
        },
        bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos},
    },
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

pub struct ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub active_outer_bitmap_iterator: ActiveOuterBitmapIterator<'a, M, B, Q, In>,

    /// The last returned outer bitmap item
    pub item: OuterBitmapItem<M, B, Q, In>,

    /// Linear iterator
    pub linear_iterator: In::OuterPosIter,

    /// Stop when limit reached
    pub limit: OuterPos<In>,
}

impl<'a, M, B, Q, In> ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn new(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterBitmapIndex<In>, OuterPos<In>)>,
    ) -> Result<Self, GoblinError> {
        let mut active_outer_bitmap_iterator =
            ActiveOuterBitmapIterator::new(market_key, range.start().0..=range.end().0);

        let item = active_outer_bitmap_iterator
            .next()
            .ok_or(GoblinError::IteratorOutOfBounds)?;

        let start = range
            .start()
            .1
            .adjust_start(item.outer_bitmap_index == range.start().0);

        let end = range
            .end()
            .1
            .adjust_limit(item.outer_bitmap_index == range.end().0);

        Ok(Self {
            active_outer_bitmap_iterator,
            item,
            linear_iterator: In::outer_pos_iter(start..=end),
            limit: range.end().1,
        })
    }

    pub fn on_limit(&self) -> bool {
        self.item.outer_bitmap_index == self.active_outer_bitmap_iterator.limit
    }
}
