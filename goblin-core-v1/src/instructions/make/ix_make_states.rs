use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        occupancy::OccupancyEnum::{Occupied, Vacant},
        update::UpdateEnum::{Decrease, Increase},
    },
    axis_helpers::{AxisMarker, MarketSpec, SlotSpec},
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

pub fn ix_make_states<MS, SS, In>(
    position: Position,
    region: MakeRegion,
    key: &SlotKey<RestingOrderPreimage<MS>>,
    resting_order: &RestingOrder,
    inner_bitmap_state: &mut InnerBitmap,
    last_positions: &mut SamePair<Position>,
) where
    MS: MarketSpec,
    SS: SlotSpec,
    In: LegMatcher,
{
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
            if resting_order_empty {
                // Opening with 0 size is no-op
                return;
            }
            key.store(resting_order);
            inner_bitmap_updater.activate();

            // Update last position if opened beyond the last stored position
            if let MakeRegion::OnLastPrice(_) | MakeRegion::Spread = region {
                let last_position = In::get_leg_mut(last_positions);
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
    }
}
