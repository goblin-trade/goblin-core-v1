use crate::{
    Ctx,
    axis::{
        leg::{Base, leg_matcher::LegMatcher},
        party::Sender,
        update::UpdateMarker,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    quantities::{BaseLots, FullPos, Ticks},
    types::StoreReader,
};

/// Add base lots to local make. Depending on `UM` lots are added or subtracted.
///
/// Base lots are converted and stored lots for the respective side.
///
/// # Convention
///
/// In: LegMarker represents the direction from perspective of taker.
/// Therefore when make<In = Base>(), we deposit `Quote`.
///
pub(crate) fn update_delta<MS: MarketSpec, UM: UpdateMarker, OP: LegMatcher>(
    delta_base_lots: BaseLots<u64>,
    position: FullPos,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let market = &ctx.readables.market_readables().market;
    let base_lot_size = Base::get(&market.lot_size_pair_u32).widen_to_u64();

    let price = Ticks::from(position);
    let price_in_quote_lots = market
        .tick_size_u32
        .widen_to_u64()
        .checked_mul(price)
        .ok_or(GoblinError::Overflow)?;

    Sender::get_leg_mut(&mut ctx.writables.local_delta)
        .make
        .add_make::<UM, OP>(delta_base_lots, base_lot_size, price_in_quote_lots)
}
