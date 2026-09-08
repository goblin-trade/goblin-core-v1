use crate::{
    Ctx,
    axis::leg::{Base, leg_matcher::LegMatcher},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    instructions::TakeHeader,
    market::MarketReadables,
    matching::{FillOutcome, match_iterator::match_iterator},
    require,
    types::StoreReader,
};

/// Match a take order
pub fn match_order<MS: MarketSpec, In: LegMatcher>(
    header: TakeHeader<In>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let MarketReadables { market, market_key } = ctx.readables.market_readables();

    let base_lot_size = Base::get(&market.lot_size_pair);
    let input_budget = In::matching_lots_taker(header.num_lots, base_lot_size)?;
    let mut budget = input_budget;

    let iterator =
        match_iterator::<MS::Pair, In>(*market_key, header.limit, &mut ctx.writables.market_state)?;
    for resting_order_entry in iterator {
        let fill_outcome =
            FillOutcome::<In>::new(market.tick_size, &resting_order_entry, &mut budget)?;

        ctx.writables
            .local_delta
            .add_take(base_lot_size, &fill_outcome)?;

        if fill_outcome.budget_exhausted {
            break;
        }
    }

    let total_matched = In::lots_taker(input_budget - budget, base_lot_size);
    require!(
        total_matched >= header.min_lots_to_fill,
        GoblinError::InsufficientTakerFill
    );

    Ok(())
}
