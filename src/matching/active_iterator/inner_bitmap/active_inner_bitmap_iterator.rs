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
        bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, Coordinate},
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
    pub outer_bitmap_item: OuterBitmapItem<M, B, Q, In>,

    /// Linear iterator
    pub outer_pos_iter: In::OuterPosIter,

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
        start_outer_bitmap_index: OuterBitmapIndex<In>,
        limit_outer_bitmap_index: OuterBitmapIndex<In>,
        start_outer_pos: OuterPos<In>,
        limit_outer_pos: OuterPos<In>,
    ) -> Result<Self, GoblinError> {
        let mut active_outer_bitmap_iterator = ActiveOuterBitmapIterator::new(
            market_key,
            start_outer_bitmap_index,
            limit_outer_bitmap_index,
        );

        if let Some(outer_bitmap_item) = active_outer_bitmap_iterator.next() {
            // Reset starting OuterPos if the starting OuterBitmapIndex is crossed
            let outer_pos = if start_outer_bitmap_index
                .closer_to_centre(outer_bitmap_item.outer_bitmap_index)
            {
                In::start_value()
            } else {
                start_outer_pos
            };

            Ok(Self {
                active_outer_bitmap_iterator,
                outer_bitmap_item,
                outer_pos_iter: In::outer_pos_iter(outer_pos),
                limit: limit_outer_pos,
            })
        } else {
            return Err(GoblinError::CallFail);
        }
    }
}
