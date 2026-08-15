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
    position: Position,
    region: MakeRegion,
    base_lots: BaseLots,
    Readables {
        msg_sender,
        market_readables,
    }: &Readables<MS>,
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
        market_key: market_readables.market_key,
        position,
    }
    .hash();

    let updated_base_lots = UM::update_resting_order::<MS, OM>(
        msg_sender,
        base_lots,
        key,
        &mut InnerBitmapUpdater {
            bitmap: inner_bitmap_state,
            pos: InnerPos::from(position),
        },
    )?;

    let base_lot_size = Base::get(&market_readables.market.lot_size_pair);
    let tick_size = market_readables.market.tick_size;
    let price = Ticks::from(position);

    writables.local_delta.make.add_make::<In, UM>(
        updated_base_lots,
        base_lot_size,
        tick_size,
        price,
    )
}
