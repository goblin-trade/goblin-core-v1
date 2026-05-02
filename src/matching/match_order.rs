use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        match_iterator_v2::{match_iterator_v2, RestingOrderEntryV2},
        region::take_region::TakeRegion,
    },
    quantities::{Position, QuantityOps, Ticks},
    require,
    settlement::local_delta::LocalDelta,
    state::MarketState,
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
    market_state: &mut MarketState,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    limit: Position,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let last_position_mut = In::get_leg_mut(&mut market_state.last_positions);
    let region = In::take_region(limit.into(), (*last_position_mut).into());

    require!(
        region == TakeRegion::Leg,
        GoblinError::TakerPriceLimitReached
    );

    let iterator = match_iterator_v2::<M, B, Q, In>(*market_key, *last_position_mut, limit);

    let base_lot_size = Base::get(&market.lot_size_pair);
    let mut budget = In::matching_lots_in(num_lots, base_lot_size);

    for RestingOrderEntryV2 {
        position,
        resting_order_key,
        mut resting_order,
    } in iterator
    {
        *last_position_mut = position;
        let price = Ticks::from(position);

        let quote = In::matching_lots_maker(resting_order.base_lots, market.tick_size, price);

        let matched = quote.min(budget);
        budget -= matched;
        local_delta.add_matched::<In>(resting_order.maker, matched, market.tick_size, price)?;

        if budget == In::MatchingLots::ZERO {
            let residue = quote - matched;
            if residue > In::MatchingLots::ZERO {
                resting_order.base_lots =
                    In::base_lots_from_matching(residue, market.tick_size, price);
                resting_order_key.store(&resting_order);
            }
            break;
        }
    }

    local_delta.verify_min_match::<In>(min_lots_to_fill, base_lot_size)
}
