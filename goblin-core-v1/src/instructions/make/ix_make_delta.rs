use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        update::UpdateMarker,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    market::{Readables, Writables},
    quantities::{BaseLots, Position, Ticks},
    types::StoreReader,
};

pub fn ix_make_delta<MS: MarketSpec, UM: UpdateMarker, In: LegMatcher>(
    delta_base_lots: BaseLots,
    position: Position,
    readables: &Readables<MS>,
    writables: &mut Writables,
) -> Result<(), GoblinError> {
    let base_lot_size = Base::get(&readables.market_readables.market.lot_size_pair);
    let tick_size = readables.market_readables.market.tick_size;
    let price = Ticks::from(position);

    writables
        .local_delta
        .make
        .add_make::<UM, In>(delta_base_lots, base_lot_size, tick_size, price)
}
