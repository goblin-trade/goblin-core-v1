use crate::{
    axis::occupancy::{OccupancyEnum, OccupancyMarker},
    axis_helpers::MarketSpec,
    quantities::{InnerPos, Position},
    state::bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
};

pub(crate) fn update_matrix<MS, OM>(
    resting_order_empty: bool,
    position: Position,
    inner_bitmap_state: &mut InnerBitmap,
) where
    MS: MarketSpec,
    OM: OccupancyMarker,
{
    let mut inner_bitmap_updater = InnerBitmapUpdater {
        bitmap: inner_bitmap_state,
        pos: InnerPos::from(position),
    };

    if resting_order_empty {
        inner_bitmap_updater.deactivate();
    }

    if OM::VARIANT == OccupancyEnum::Vacant {
        inner_bitmap_updater.activate();
    }
}
