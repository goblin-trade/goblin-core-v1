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
            inner_pos::InnerPos, range::Range, CoordinatesRange, FullCoordinates, StoredCoordinates,
        },
    },
    quantities::Ticks,
    require,
    state::{MarketPreimage, SlotKey},
};

pub struct CoordinateIterator<'a, M, B, Q, In>
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
    pub linear_iterator: In::InnerPosIter,

    pub limit: InnerPos<In>,
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
        Self::new_inner(
            market_key,
            Range {
                start: FullCoordinates::from(last_coordinate),
                limit: FullCoordinates::from(price_limit),
            },
        )
    }

    fn new_inner(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        coordinates_range: Range<FullCoordinates<In>>,
    ) -> Result<Self, GoblinError> {
        let range = CoordinatesRange::<In>::from(coordinates_range);
        let mut active_inner_bitmap_iterator =
            ActiveInnerBitmapIterator::new(market_key, range.outer_bitmap_index, range.outer_pos)?;

        let item = active_inner_bitmap_iterator
            .next()
            .ok_or(GoblinError::IteratorOutOfBounds)?;

        let adjusted_range = range.inner_pos.adjust(
            (item.outer_bitmap_index, item.outer_pos),
            Range {
                start: (range.outer_bitmap_index.start, range.outer_pos.start),
                limit: (range.outer_bitmap_index.limit, range.outer_pos.limit),
            },
        );

        Ok(Self {
            active_inner_bitmap_iterator,
            item,
            linear_iterator: In::inner_pos_iter(adjusted_range),
            limit: range.inner_pos.limit,
        })
    }
}
