use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        occupancy::{
            OccupancyEnum::{Occupied, Vacant},
            OccupancyMarker,
        },
        update::{
            update_make::UpdateMake,
            UpdateEnum::{Decrease, Increase},
        },
    },
    axis_helpers::{AxisMarker, MarketSpec, SlotSpec},
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        Preimage,
    },
    Ctx,
};

pub fn ix_make_states<MS, SS, In>(
    base_lots: BaseLots,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut InnerBitmap,
    ctx: &mut Ctx<MS>,
) -> Result<BaseLots, GoblinError>
where
    MS: MarketSpec,
    SS: SlotSpec,
    In: LegMatcher,
{
    SS::Occupancy::validate_region::<In>(region, position, inner_bitmap_state)?;

    let key = &RestingOrderPreimage {
        market_key: ctx.readables.market_readables().market_key,
        position,
    }
    .hash();

    let resting_order =
        &mut SS::Occupancy::get_validated_resting_order(key, ctx.readables.msg_sender)?;
    let delta_base_lots = SS::Update::update_resting_order(base_lots, resting_order)?;

    let resting_order_empty = resting_order.base_lots == BaseLots::default();

    if !resting_order_empty {
        key.store(resting_order);
    }

    let mut inner_bitmap_updater = InnerBitmapUpdater {
        bitmap: inner_bitmap_state,
        pos: InnerPos::from(position),
    };

    match (SS::Occupancy::VARIANT, SS::Update::VARIANT) {
        (Vacant, Increase) => {
            // illegal, unreachable
        }
        (Vacant, Decrease) => {
            key.store(resting_order);
            inner_bitmap_updater.activate();

            // Update last position if opened beyond the last stored position
            if let MakeRegion::OnLastPrice(_) | MakeRegion::Spread = region {
                let last_position = In::get_leg_mut(&mut ctx.writables.market_state.last_positions);
                *last_position = position;
            }
        }
        (Occupied, Increase) => {
            if resting_order_empty {
                inner_bitmap_updater.deactivate();
            } else {
                key.store(resting_order);
            }
        }
        (Occupied, Decrease) => {
            key.store(resting_order);
        }
    };

    Ok(delta_base_lots)
}
