use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    instructions::TakeHeader,
    market::MarketReadables,
    matching::match_iterator::{match_iterator, RestingOrderEntry},
    quantities::Ticks,
    require,
    settlement::ConstDefault,
    state::resting_order::RestingOrder,
    types::StoreReader,
    Ctx,
};

/// Match a take order
///
/// We match against resting orders until one of these conditions is met
/// * The budget is consumed
/// * Price limit reached
/// * All resting orders are popped
///
pub fn match_order<MS: MarketSpec, In: LegMatcher>(
    TakeHeader {
        num_lots,
        min_lots_to_fill,
        limit,
    }: TakeHeader<In>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let MarketReadables { market, market_key } = ctx.readables.market_readables();

    // TODO common struct in Market for sizes

    let last_position_mut = In::get_leg_mut(&mut ctx.writables.market_state.last_positions);

    require!(
        In::in_region(*last_position_mut, limit),
        GoblinError::TakerPriceLimitReached
    );

    let iterator = match_iterator::<MS::Pair, In>(*market_key, *last_position_mut, limit);

    let base_lot_size = Base::get(&market.lot_size_pair);
    let input_budget = In::matching_lots_taker(num_lots, base_lot_size);

    let mut budget = input_budget;

    for RestingOrderEntry {
        position,
        mut resting_order_key_value,
    } in iterator
    {
        let RestingOrder {
            base_lots,
            maker: counterparty,
        } = resting_order_key_value.value;

        *last_position_mut = position;

        let price = Ticks::from(position);
        let price_in_quote_lots = market.tick_size * price;

        let quote = In::matching_lots_maker(base_lots, price_in_quote_lots);
        let matched = quote.min(budget);

        budget -= matched;

        ctx.writables.local_delta.add_take::<In>(
            &counterparty,
            matched,
            base_lot_size,
            price_in_quote_lots,
        )?;

        if budget == In::MatchingLots::DEFAULT {
            let residue = quote - matched;
            if residue > In::MatchingLots::DEFAULT {
                resting_order_key_value.value.base_lots =
                    In::base_lots_maker(residue, price_in_quote_lots);

                resting_order_key_value.store();
            }
            break;
        }
    }

    let total_matched = In::lots_taker(input_budget - budget, base_lot_size);
    require!(
        total_matched >= min_lots_to_fill,
        GoblinError::InsufficientTakerFill
    );

    Ok(())
}
