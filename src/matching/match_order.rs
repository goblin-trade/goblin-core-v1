use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketReadables, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::TakeHeader,
    matching::match_iterator::{match_iterator, RestingOrderEntry},
    quantities::Ticks,
    require,
    settlement::ConstZero,
    state::resting_order::RestingOrder,
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
    TakeHeader {
        num_lots,
        min_lots_to_fill,
        limit,
    }: TakeHeader<In>,
    readables: &Readables<M, B, Q>,
    writables: &mut Writables,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let MarketReadables { market, market_key } = readables.market_readables;

    let last_position_mut = In::get_leg_mut(&mut writables.market_state.last_positions);

    require!(
        In::in_region(*last_position_mut, limit),
        GoblinError::TakerPriceLimitReached
    );

    let iterator = match_iterator::<M, B, Q, In>(*market_key, *last_position_mut, limit);

    let base_lot_size = Base::get(&market.lot_size_pair);
    let input_budget = In::matching_lots_in(num_lots, base_lot_size);

    let mut budget = input_budget;

    for RestingOrderEntry {
        position,
        mut resting_order_key_value,
    } in iterator
    {
        let RestingOrder { base_lots, maker } = resting_order_key_value.value;

        *last_position_mut = position;
        let price = Ticks::from(position);

        // TODO cleanup- common struct for base lots, tick size, price
        // also for matching lots, tick size, price
        let quote = In::matching_lots_maker(base_lots, market.tick_size, price);

        let matched = quote.min(budget);
        budget -= matched;

        writables.local_delta.take.add_take::<In>(
            &maker,
            matched,
            base_lot_size,
            market.tick_size,
            price,
        )?;

        if budget == In::MatchingLots::ZEROED {
            let residue = quote - matched;
            if residue > In::MatchingLots::ZEROED {
                resting_order_key_value.value.base_lots =
                    In::base_lots_from_matching(residue, market.tick_size, price);

                resting_order_key_value.store();
            }
            break;
        }
    }

    let total_matched = In::decode_matching_lots(input_budget - budget, base_lot_size);
    require!(
        total_matched >= min_lots_to_fill,
        GoblinError::InsufficientTakerFill
    );

    Ok(())
}
