use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{active_iterator::coordinate::CoordinateIterator, bitmap::StoredCoordinates},
    quantities::{BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerQuoteUnit, Ticks},
    settlement::{
        local_delta::{LocalDelta, MakerDelta},
        MatchedLots,
    },
    state::{MarketState, Preimage},
    types::{Address, StoreReader, Tuple},
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
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    price_limit: Ticks,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>, Result = MatchedLots<In>>
        + StoreReader<
            Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>,
            Result = In::LotsPerUnit,
        > + StoreReader<Tuple<StoredCoordinates, StoredCoordinates, Leg>, Result = StoredCoordinates>,
{
    let MarketAndKey { market, market_key } = market_and_key;

    let last_coordinate = In::get_leg_mut(&mut market_state.last_coordinates);
    let iterator =
        CoordinateIterator::<M, B, Q, In>::new(market_key, *last_coordinate, price_limit)?;

    let base_lot_size = Base::get(&market.lot_size_pair);
    let mut budget = In::matching_lots_taker(num_lots, base_lot_size);

    for item in iterator {
        // Update last market price
        *last_coordinate = item.full_coordinates.into();
        if budget == In::MatchingLots::ZERO {
            break;
        }

        let preimage = item.preimage();
        let hash = preimage.hash();
        let mut resting_order = hash.load();

        let quote =
            In::matching_lots_maker(resting_order.size, market.tick_size, last_coordinate.price);

        let matched_lots =
            MatchedLots::<In>::new(quote, budget, market.tick_size, last_coordinate.price);

        local_delta.add_matched(resting_order.maker, matched_lots)?;
        budget -= matched_lots.taker_in;

        // Budget exhausted but maker residue remains
        // Write updated resting order to slot
        if quote > matched_lots.taker_in {
            let residue = quote - matched_lots.taker_in;
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
