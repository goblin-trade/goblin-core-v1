use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        update::update_make::UpdateMake,
    },
    axis_helpers::{MarketSpec, SlotSpec},
    goblin_error::GoblinError,
    instructions::make::{ix_make_delta::ix_make_delta, ix_make_states::ix_make_states},
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position},
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
};

pub fn ix_make_inner<MS: MarketSpec, SS: SlotSpec, In: LegMatcher>(
    base_lots: BaseLots,
    position: Position,
    region: MakeRegion,
    // TODO common struct wrapper for Readables and Writables
    readables: &Readables<MS>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    SS::Occupancy::validate_region::<In>(region, position, inner_bitmap_state)?;

    let key = &RestingOrderPreimage {
        market_key: readables.market_readables.market_key,
        position,
    }
    .hash();

    let resting_order = &mut SS::Occupancy::get_validated_resting_order(key, readables.msg_sender)?;
    let delta_base_lots = SS::Update::update_resting_order(base_lots, resting_order)?;

    // 1. Update states
    ix_make_states::<MS, SS, In>(
        position,
        region,
        key,
        resting_order,
        inner_bitmap_state,
        &mut writables.market_state.last_positions,
    );

    // 2. Update delta
    ix_make_delta::<MS, SS::Update, In>(delta_base_lots, position, readables, writables)
}
