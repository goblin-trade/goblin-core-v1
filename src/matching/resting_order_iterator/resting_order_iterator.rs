use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Leg, Pair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::bitmap::{Column, Coordinates},
    quantities::{QuantityOps, Ticks},
    require,
    state::{MarketPreimage, SlotKey},
    types::{StoreReader, Tuple},
};

/// Iterator for resting order positions
///
/// In: LegMatcher denotes the taker side.
pub struct RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
    pub coordinates: Coordinates,

    pub _marker: core::marker::PhantomData<In>,
}

impl<'a, M, B, Q, In> RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    /// Create a new RestingOrderIterator. Fail if best price crosses price limit.
    pub fn new(
        market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
        best_prices: &Pair<Ticks, Ticks>,
        price_limit: Ticks,
        min_lots_to_fill: In::Lots,
    ) -> Result<Self, GoblinError>
    where
        In::Opposite: StoreReader<Tuple<Ticks, Ticks, Leg>, Result = Ticks>,
    {
        let last_opposite_price = In::Opposite::get(best_prices);

        require!(
            In::Opposite::closer_to_centre(last_opposite_price, price_limit)
                || min_lots_to_fill == In::Lots::ZERO,
            GoblinError::TakerPriceLimitReached
        );

        let coordinates = Coordinates {
            price_coordinates: last_opposite_price.into(),
            column: Column::ZERO,
        };

        Ok(Self {
            market_key,
            coordinates,
            _marker: core::marker::PhantomData,
        })
    }
}
