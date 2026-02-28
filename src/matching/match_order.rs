use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::coordinate::CoordinateIterator,
        bitmap::{range::Range, FullCoordinates, StoredCoordinates},
    },
    quantities::{BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerQuoteUnit, Ticks},
    require,
    settlement::{
        local_delta::{LocalDelta, MakerDelta, TakerDelta},
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
    taker: &Address,
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
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>
        + StoreReader<
            Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>,
            Result = In::LotsPerUnit,
        >,
    In: StoreReader<Tuple<StoredCoordinates, StoredCoordinates, Leg>, Result = StoredCoordinates>,
{
    let MarketAndKey {
        market,
        key: market_key,
    } = market_and_key;

    let base_lot_size = Base::get(&market.lot_size_pair);
    let budget = In::matching_lots_taker(num_lots, base_lot_size);

    let taker_delta = In::get_leg_mut(&mut local_delta.local_sender_delta.taker_delta_pair);
    *taker_delta = TakerDelta::<In>::zero();

    // Convention- market_state.last_prices<In> means te opposite price matched
    let last_coordinate = In::get_leg_mut(&mut market_state.last_coordinates);

    if In::closer_to_centre(price_limit, last_coordinate.price) {
        require!(
            min_lots_to_fill == In::Lots::ZERO,
            GoblinError::TakerPriceLimitReached
        );

        return Ok(());
    }

    let mut iterator = CoordinateIterator::<M, B, Q, In>::new(
        market_key,
        Range {
            start: FullCoordinates::from(*last_coordinate),
            limit: FullCoordinates::from(price_limit),
        },
    )?;

    let mut remaining_budget = budget;

    while remaining_budget >= In::MatchingLots::ZERO {
        if let Some(item) = iterator.next() {
            *last_coordinate = item.full_coordinates.into();
            if remaining_budget == In::MatchingLots::ZERO {
                break;
            }

            let preimage = item.preimage();
            let hash = preimage.hash();
            let mut resting_order = hash.load();

            let quote = In::matching_lots_maker(
                resting_order.size,
                market.tick_size,
                last_coordinate.price,
            );

            let matched = remaining_budget.min(quote);
            let matched_opposite =
                In::opposite_matching_lots(matched, market.tick_size, last_coordinate.price);

            let matched_lots_delta = MatchedLots::<In> {
                taker_in: matched,
                taker_out: matched_opposite,
            };
            // Update taker
            taker_delta
                .matched_lots
                .checked_add_v3(matched_lots_delta)
                .ok_or(GoblinError::DeltaOverflow)?;

            // Update maker
            let maker_delta_pair = local_delta
                .local_maker_deltas
                .get_or_insert_mut(resting_order.maker)
                .ok_or(GoblinError::MakerListFull)?;

            let maker_delta = In::get_leg_mut(maker_delta_pair);
            maker_delta
                .matched_lots
                .checked_add_v3(matched_lots_delta)
                .ok_or(GoblinError::DeltaOverflow)?;

            remaining_budget -= matched;

            if matched < quote {
                let residue = quote - matched;
                let residue_base_lots =
                    In::base_lots_from_matching(residue, market.tick_size, last_coordinate.price);

                resting_order.size = residue_base_lots;
                hash.store(&resting_order);
                break;
            }
        } else {
            break;
        }
    }

    let min_lots = In::matching_lots_taker(min_lots_to_fill, base_lot_size);

    require!(
        taker_delta.matched_lots.taker_in >= min_lots,
        GoblinError::InsufficientTakerFill
    );

    Ok(())
}
