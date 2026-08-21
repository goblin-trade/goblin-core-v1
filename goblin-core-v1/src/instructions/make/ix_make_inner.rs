use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, occupancy::occupancy_marker::OccupancyMarker,
        update::update_make::UpdateMake,
    },
    axis_helpers::{MarketSpec, SlotSpec},
    goblin_error::GoblinError,
    instructions::make::{ix_make_delta::ix_make_delta, ix_make_states::ix_make_states},
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position},
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
    Ctx,
};

pub fn ix_make_inner<MS: MarketSpec, SS: SlotSpec, In: LegMatcher>(
    base_lots: BaseLots,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut InnerBitmap,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    SS::Occupancy::validate_region::<In>(region, position, inner_bitmap_state)?;

    let key = &RestingOrderPreimage {
        market_key: ctx.readables.market_readables().market_key,
        position,
    }
    .hash();

    let resting_order =
        &mut SS::Occupancy::get_validated_resting_order(key, ctx.readables.msg_sender)?;
    let delta_base_lots = SS::Update::update_resting_order(base_lots, resting_order)?;

    // 1. Update states
    // TODO reduce param count. This function can return delta_base_lots
    ix_make_states::<MS, SS, In>(
        position,
        region,
        key,
        resting_order,
        inner_bitmap_state,
        &mut ctx.writables.market_state.last_positions,
    );

    // 2. Update delta
    ix_make_delta::<MS, SS::Update, In>(delta_base_lots, position, ctx)
}
