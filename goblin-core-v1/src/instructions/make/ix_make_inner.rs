use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_spec::MarketSpec, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position, Ticks},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        Preimage,
    },
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

    let key = &mut RestingOrderPreimage {
        market_key: readables.market_readables.market_key,
        position,
    }
    .hash();

    let resting_order = &mut OM::get_validated_resting_order(key, readables.msg_sender)?;
    let delta_base_lots = UM::update_resting_order(base_lots, resting_order)?;

    let resting_order_closed = resting_order.base_lots == BaseLots::default();

    if !resting_order_closed {
        key.store(resting_order);
    } else {
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
