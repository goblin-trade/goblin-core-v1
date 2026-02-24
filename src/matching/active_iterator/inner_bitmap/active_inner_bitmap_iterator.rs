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
        bitmap::{
            outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, range::Range, Coordinate,
        },
    },
    state::{MarketPreimage, SlotKey},
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
        outer_bitmap_index_range: Range<OuterBitmapIndex<In>>,
        outer_pos_range: Range<OuterPos<In>>,
    ) -> Result<Self, GoblinError> {
        let mut active_outer_bitmap_iterator =
            ActiveOuterBitmapIterator::new(market_key, outer_bitmap_index_range);

        if let Some(outer_bitmap_item) = active_outer_bitmap_iterator.next() {
            // Reset starting OuterPos if the starting OuterBitmapIndex is crossed
            let outer_pos = if outer_bitmap_index_range
                .start
                .closer_to_centre(outer_bitmap_item.outer_bitmap_index)
            {
                In::start_value()
            } else {
                outer_pos_range.start
            };

            Ok(Self {
                active_outer_bitmap_iterator,
                item: outer_bitmap_item,
                linear_iterator: In::outer_pos_iter(outer_pos),
                limit: outer_pos_range.limit,
            })
        } else {
            return Err(GoblinError::CallFail);
        }
    }
}
