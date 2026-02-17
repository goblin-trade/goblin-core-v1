use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Leg, Pair},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::bitmap::{column::Column, Coordinates},
    quantities::{QuantityOps, Ticks},
    require,
    state::{
        outer_bitmap::{
            outer_bitmap, outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage,
            OuterBitmap,
        },
        MarketPreimage, Preimage, SlotKey,
    },
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
    pub coordinates: Coordinates<In>,
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,
    pub outer_bitmap_state: OuterBitmapState<M, B, Q>,

    // TODO add price limit
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

        let mut coordinates = Coordinates {
            price_coordinates: last_opposite_price.into(),
            column: Column::new(0),
        };

        loop {
            let outer_bitmap_preimage = OuterBitmapPreimage {
                market_key: *market_key,
                outer_bitmap_index: coordinates.price_coordinates.outer_bitmap_index,
            };
            let outer_bitmap_key = outer_bitmap_preimage.hash();
            let outer_bitmap = outer_bitmap_key.load();
            let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

            match outer_bitmap_state {
                OuterBitmapState::Closed(_) => {
                    // TODO advance the OuterBitmapIndex
                    // Ask- increase, bid- decrease
                    // Attach generic?

                    continue;
                }
                OuterBitmapState::Active(active_outer_bitmap) => {
                    return Ok(Self {
                        market_key,
                        coordinates,
                        outer_bitmap_key,
                        outer_bitmap_state: OuterBitmapState::Active(active_outer_bitmap),
                        _marker: core::marker::PhantomData,
                    });
                }
            }
        }

        // We store outer bitmap and inner bitmap keys here. This way we don't have
        // to derive hash each time next() is called
        //
        // Problem with InnerBitmap
        // * If outer_bitmap_state is empty or if OuterPos is inactive, we need to
        // iterate to the next position
        //
        // We end up duplicating logic from next() in new()
        // Keep on iterating till the first position is found
        //
        // Alt design- Use Option<> or MaybeUninit<>
    }

    // fn outer_bitmap_preimage(&self) -> OuterBitmapPreimage<M, B, Q> {
    //     Self {
    //         market_key: *&self.market_key,
    //     }
    // }
}
