use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketReadables, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::TakeHeader,
    matching::match_iterator::{match_iterator, RestingOrderEntry},
    quantities::{QuantityOps, Ticks},
    require,
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
    let mut budget = In::matching_lots_in(num_lots, base_lot_size);

    for RestingOrderEntry {
        position,
        resting_order_key,
        mut resting_order,
    } in iterator
    {
        *last_position_mut = position;
        let price = Ticks::from(position);

        // TODO cleanup- common struct for base lots, tick size, price
        // also for matching lots, tick size, price
        let quote = In::matching_lots_maker(resting_order.base_lots, market.tick_size, price);

        let matched = quote.min(budget);
        budget -= matched;
        writables.local_delta.add_matched::<In>(
            resting_order.maker,
            matched,
            market.tick_size,
            price,
        )?;

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

    writables
        .local_delta
        .local_sender_delta
        .verify_min_match::<In>(min_lots_to_fill, base_lot_size)
}
