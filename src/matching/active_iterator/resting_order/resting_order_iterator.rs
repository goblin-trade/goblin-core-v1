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
            column::Column, inner_coordinates::InnerCoordinates, range::Range, row::Row,
            Coordinate, CoordinatesRange,
        },
    },
    quantities::Ticks,
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
    pub item: InnerBitmapItem<M, B, Q, In>,

    /// Linear iterator
    pub linear_iterator: In::CoordinatesIter,

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
        price_range: Range<Ticks>,
    ) -> Result<Self, GoblinError> {
        let range = CoordinatesRange::<In>::from(price_range);
        let mut active_inner_bitmap_iterator =
            ActiveInnerBitmapIterator::new(market_key, range.outer_bitmap_index, range.outer_pos)?;

        if let Some(inner_bitmap_item) = active_inner_bitmap_iterator.next() {
            // Reset starting Row if the starting OuterBitmapIndex or OuterPos is crossed
            let row = if range
                .outer_bitmap_index
                .start
                .closer_to_centre(inner_bitmap_item.outer_bitmap_index)
                || (range.outer_bitmap_index.start == range.outer_bitmap_index.limit
                    && range
                        .outer_pos
                        .start
                        .closer_to_centre(inner_bitmap_item.outer_pos))
            {
                In::start_value()
            } else {
                range.row.start
            };

            Ok(Self {
                active_inner_bitmap_iterator,
                item: inner_bitmap_item,
                linear_iterator: In::coordinates_iter(
                    InnerCoordinates {
                        row,
                        column: Column::new(0),
                    }
                    .into(),
                ),
                limit: range.row.limit,
            })
        } else {
            return Err(GoblinError::CallFail);
        }
    }
}
