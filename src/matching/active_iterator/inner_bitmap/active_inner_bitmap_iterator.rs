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
    /// The last returned outer bitmap item
    pub inner_item: OuterBitmapItem<M, B, Q, In>,

    pub inner_iterator: ActiveOuterBitmapIterator<'a, M, B, Q, In>,

    /// Stop when limit reached
    pub limit: OuterPos<In>,

    /// Linear iterator
    pub linear_iterator: In::OuterPosIter,
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
        let mut inner_iterator =
            ActiveOuterBitmapIterator::new(market_key, range.start().0..=range.end().0);

        let inner_item = inner_iterator
            .next()
            .ok_or(GoblinError::IteratorOutOfBounds)?;

        let start = range
            .start()
            .1
            .adjust_start(inner_item.outer_bitmap_index == range.start().0);

        let end = range
            .end()
            .1
            .adjust_limit(inner_item.outer_bitmap_index == range.end().0);

        let linear_iterator = In::outer_pos_iter(start..=end);

        Ok(Self {
            inner_iterator,
            inner_item,
            linear_iterator,
            limit: range.end().1,
        })
    }

    pub fn update_inner_item(&mut self) -> Option<()> {
        // Can this function used in both new() and next()
        // We don't store 'start' in the iterator, this value is lost.
        self.inner_item = self.inner_iterator.next()?;
        let end = self
            .limit
            .adjust_limit(self.inner_item.outer_bitmap_index == self.inner_iterator.limit);
        self.linear_iterator = In::outer_pos_iter(In::start_value()..=end);

        Some(())
    }

    pub fn on_limit(&self) -> bool {
        self.inner_item.outer_bitmap_index == self.inner_iterator.limit
    }
}
