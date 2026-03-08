use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::active_iterator::coordinate::CoordinateIterator,
    quantities::{QuantityOps, Ticks},
    settlement::{local_delta::LocalDelta, MatchedLots},
    state::{MarketState, Preimage},
    types::StoreReader,
};

/// Match a take order
///
/// We match against resting orders until one of these conditions is met
/// * The budget is consumed
/// * Price limit reached
/// * All resting orders are popped
///
pub fn match_order<M, B, Q, In>(
    local_delta: &mut LocalDelta,
    MarketAndKey { market, market_key }: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    price_limit: Ticks,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let last_coordinate = In::get_leg_mut(&mut market_state.last_coordinates);
    let mut iterator =
        CoordinateIterator::<M, B, Q, In>::new(market_key, *last_coordinate, price_limit)?;

    let base_lot_size = Base::get(&market.lot_size_pair);
    let mut budget = In::matching_lots_taker(num_lots, base_lot_size);

    for item in iterator {
        *last_coordinate = item.full_coordinates.into();
        if budget == In::MatchingLots::ZERO {
            break;
        }

        let preimage = item.preimage();
        let hash = preimage.hash();
        let mut resting_order = hash.load();

        let quote =
            In::matching_lots_maker(resting_order.size, market.tick_size, last_coordinate.price);

        let matched = budget.min(quote);

        local_delta.add_matched::<In>(
            resting_order.maker,
            matched,
            market.tick_size,
            last_coordinate.price,
        )?;

        // Since we use taker_in = min(budget, quote) this subtraction never underflows
        // But the code is unclear
        budget -= matched;

        if quote > matched {
            let residue = quote - matched;
            let residue_base_lots =
                In::base_lots_from_matching(residue, market.tick_size, last_coordinate.price);

            resting_order.size = residue_base_lots;
            hash.store(&resting_order);
            break;
        }
    }

    local_delta.verify_min_match::<In>(min_lots_to_fill, base_lot_size)?;

    Ok(())
}
