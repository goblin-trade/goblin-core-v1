use crate::{
    axis::leg::{leg_matcher::LegMatcher, Leg, Pair},
    goblin_error::GoblinError,
    matching::resting_order_iterator::resting_order_position::RestingOrderPosition,
    quantities::{QuantityOps, Ticks},
    require,
    types::{StoreReader, Tuple},
};

/// Iterator for resting order positions
///
/// In: LegMatcher denotes the taker side.
pub struct RestingOrderIterator<'a, In: LegMatcher> {
    pub best_opposite_price: &'a mut Ticks,
    _marker: core::marker::PhantomData<In>,
}

impl<'a, In: LegMatcher> RestingOrderIterator<'a, In> {
    /// Create a new RestingOrderIterator. Fail if best price crosses price limit.
    pub fn new(
        best_prices: &'a mut Pair<Ticks, Ticks>,
        price_limit: Ticks,
        min_lots_to_fill: In::Lots,
    ) -> Result<Self, GoblinError>
    where
        In::Opposite: StoreReader<Tuple<Ticks, Ticks, Leg>, Result = Ticks>,
    {
        let best_opposite_price = In::Opposite::get_leg_mut(best_prices);

        require!(
            In::Opposite::closer_to_centre(*best_opposite_price, price_limit)
                || min_lots_to_fill == In::Lots::ZERO,
            GoblinError::TakerPriceLimitReached
        );

        Ok(Self {
            best_opposite_price,
            _marker: core::marker::PhantomData,
        })
    }
}

impl<'a, In: LegMatcher> Iterator for RestingOrderIterator<'a, In> {
    type Item = RestingOrderPosition;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
