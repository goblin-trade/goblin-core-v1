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
            column::Column, inner_coordinates::InnerCoordinates,
            outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row, Coordinate,
        },
    },
    state::{MarketPreimage, SlotKey},
};

pub struct RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub active_inner_bitmap_iterator: ActiveInnerBitmapIterator<'a, M, B, Q, In>,

    /// The last returned inner bitmap item
    pub inner_bitmap_item: InnerBitmapItem<M, B, Q, In>,

    /// Linear iterator
    pub coordinates_iter: In::CoordinatesIter,

    pub limit: Row<In>,
}

impl<'a, M, B, Q, In> RestingOrderIterator<'a, M, B, Q, In>
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
        start_row: Row<In>,
        limit_row: Row<In>,
    ) -> Result<Self, GoblinError> {
        let mut active_inner_bitmap_iterator = ActiveInnerBitmapIterator::new(
            market_key,
            start_outer_bitmap_index,
            limit_outer_bitmap_index,
            start_outer_pos,
            limit_outer_pos,
        )?;

        if let Some(inner_bitmap_item) = active_inner_bitmap_iterator.next() {
            // Reset starting Row if the starting OuterBitmapIndex or OuterPos is crossed
            let row = if start_outer_bitmap_index
                .closer_to_centre(inner_bitmap_item.outer_bitmap_index)
                || (start_outer_bitmap_index == limit_outer_bitmap_index
                    && start_outer_pos.closer_to_centre(inner_bitmap_item.outer_pos))
            {
                In::start_value()
            } else {
                start_row
            };

            Ok(Self {
                active_inner_bitmap_iterator,
                inner_bitmap_item,
                coordinates_iter: In::coordinates_iter(
                    InnerCoordinates {
                        row,
                        column: Column::new(0),
                    }
                    .into(),
                ),
                limit: limit_row,
            })
        } else {
            return Err(GoblinError::CallFail);
        }
    }
}
