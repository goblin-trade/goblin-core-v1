use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::inner_bitmap::{
            inner_bitmap_item::InnerBitmapItem, ActiveInnerBitmapIterator,
        },
        bitmap::{
            inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
            StoredCoordinates,
        },
    },
    quantities::Ticks,
    require,
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

pub struct CoordinateIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// The last returned inner bitmap item
    pub inner_item: InnerBitmapItem<M, B, Q, In>,

    pub inner_iterator: ActiveInnerBitmapIterator<'a, M, B, Q, In>,

    pub limit: InnerPos<In>,

    /// Linear iterator
    pub linear_iterator: In::InnerPosIter,
}

impl<'a, M, B, Q, In> CoordinateIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn new(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        last_coordinate: StoredCoordinates,
        price_limit: Ticks,
    ) -> Result<Self, GoblinError> {
        require!(
            In::closer_to_centre(last_coordinate.price, price_limit),
            GoblinError::TakerPriceLimitReached
        );
        Self::new_inner(market_key, last_coordinate.into()..=price_limit.into())
    }

    fn new_inner(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterBitmapIndex<In>, OuterPos<In>, InnerPos<In>)>,
    ) -> Result<Self, GoblinError> {
        let start_inner = (range.start().0, range.start().1);
        let end_inner = (range.end().0, range.end().1);
        let mut active_inner_bitmap_iterator =
            ActiveInnerBitmapIterator::new(market_key, start_inner..=end_inner)?;

        let item = active_inner_bitmap_iterator
            .next()
            .ok_or(GoblinError::IteratorOutOfBounds)?;

        let start = range
            .start()
            .2
            .adjust_start((item.outer_bitmap_index, item.outer_pos) == start_inner);
        let end = range
            .end()
            .2
            .adjust_end((item.outer_bitmap_index, item.outer_pos) == end_inner);

        Ok(Self {
            inner_iterator: active_inner_bitmap_iterator,
            inner_item: item,
            linear_iterator: In::inner_pos_iter(start..=end),
            limit: range.end().2,
        })
    }

    pub fn on_limit(&self) -> bool {
        self.inner_iterator.on_limit() && self.inner_item.outer_pos == self.inner_iterator.limit
    }
}
