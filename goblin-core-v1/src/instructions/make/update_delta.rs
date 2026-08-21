use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
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

    // TODO common function on Market to get lot size pair and tick size
    let base_lot_size = Base::get(&market.lot_size_pair);
    let tick_size = market.tick_size;
    let price = Ticks::from(position);

    ctx.writables.local_delta.make.add_make::<UM, In>(
        delta_base_lots,
        base_lot_size,
        tick_size,
        price,
    )
}
