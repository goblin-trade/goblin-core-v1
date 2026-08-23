use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        party::Sender,
        update::UpdateMarker,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, Ticks},
    types::StoreReader,
    Ctx,
};

pub(crate) fn update_delta<MS: MarketSpec, UM: UpdateMarker, In: LegMatcher>(
    delta_base_lots: BaseLots,
    position: Position,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let market = &ctx.readables.market_readables().market;

    let base_lot_size = Base::get(&market.lot_size_pair);
    let tick_size = market.tick_size;
    let price = Ticks::from(position);

    Sender::get_leg_mut(&mut ctx.writables.local_delta)
        .make
        .add_make::<UM, In>(delta_base_lots, base_lot_size, tick_size, price)
}
