use crate::{
    Ctx,
    axis::leg::{Base, leg_matcher::LegMatcher},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    instructions::TakeHeader,
    market::{LotSizePair, MarketReadables},
    matching::{FillOutcome, match_iterator::match_iterator},
    quantities::ScaledPosition,
    require,
    types::StoreReader,
};

/// Match a take order
pub fn match_order<MS: MarketSpec, In: LegMatcher>(
    header: TakeHeader<In>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let MarketReadables { market, market_key } = ctx.readables.market_readables();
    let base_lot_size = Base::get(&LotSizePair::<u64>::from(&market.lot_size_pair_u32));

    let num_lots = header.num_lots_u32.into();
    require!(num_lots > In::Lots::default(), GoblinError::InvalidTakeArgs);

    let input_budget = In::matching_lots_taker(num_lots, base_lot_size)?;
    let mut budget = input_budget;

    let iterator = match_iterator::<MS::Pair, In>(
        *market_key,
        header.limit_u32.scale_up(),
        &mut ctx.writables.market_state,
    )?;
    for resting_order_entry in iterator {
        let fill_outcome = FillOutcome::<In>::new(
            market.tick_size_u32.widen_to_u64(),
            &resting_order_entry,
            &mut budget,
        )?;

        ctx.writables
            .local_delta
            .add_take(base_lot_size, &fill_outcome)?;

        if fill_outcome.budget_exhausted {
            break;
        }
    }

    let total_matched = In::lots_taker(input_budget - budget, base_lot_size);
    require!(
        total_matched >= header.min_lots_to_fill_u32.into(),
        GoblinError::InsufficientTakerFill
    );

    Ok(())
}
