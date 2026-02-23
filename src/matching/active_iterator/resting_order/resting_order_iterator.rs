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
            outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, range::Range, row::Row,
            Coordinate,
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
        outer_bitmap_index_range: Range<OuterBitmapIndex<In>>,
        outer_pos_range: Range<OuterPos<In>>,
        row_range: Range<Row<In>>,
    ) -> Result<Self, GoblinError> {
        let mut active_inner_bitmap_iterator =
            ActiveInnerBitmapIterator::new(market_key, outer_bitmap_index_range, outer_pos_range)?;

        if let Some(inner_bitmap_item) = active_inner_bitmap_iterator.next() {
            // Reset starting Row if the starting OuterBitmapIndex or OuterPos is crossed
            let row = if outer_bitmap_index_range
                .start
                .closer_to_centre(inner_bitmap_item.outer_bitmap_index)
                || (outer_bitmap_index_range.start == outer_bitmap_index_range.limit
                    && outer_pos_range
                        .start
                        .closer_to_centre(inner_bitmap_item.outer_pos))
            {
                In::start_value()
            } else {
                row_range.start
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
                limit: row_range.limit,
            })
        } else {
            return Err(GoblinError::CallFail);
        }
    }
}
