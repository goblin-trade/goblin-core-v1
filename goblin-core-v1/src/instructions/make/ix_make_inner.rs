use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_spec::MarketSpec, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        update::{update_make::UpdateResult, UpdateMarker},
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position, Ticks},
    state::bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
    types::StoreReader,
};

pub fn ix_make_inner<MS: MarketSpec, In: LegMatcher, UM: UpdateMarker, OM: OccupancyMarker>(
    base_lots: BaseLots,
    position: Position,
    region: MakeRegion,
    readables: &Readables<MS>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    OM::validate_and_update_region::<In>(
        region,
        position,
        &mut writables.market_state.last_positions,
        inner_bitmap_state,
    )?;

    // number of lots to increase or decrease from global balance store
    let UpdateResult {
        delta_base_lots,
        resting_order_closed,
    } = UM::update_resting_order::<MS, OM>(base_lots, position, readables)?;

    if resting_order_closed {
        InnerBitmapUpdater {
            bitmap: inner_bitmap_state,
            pos: InnerPos::from(position),
        }
        .deactivate();
    }

    let base_lot_size = Base::get(&readables.market_readables.market.lot_size_pair);
    let tick_size = readables.market_readables.market.tick_size;
    let price = Ticks::from(position);

    writables
        .local_delta
        .make
        .add_make::<In, UM>(delta_base_lots, base_lot_size, tick_size, price)
}
