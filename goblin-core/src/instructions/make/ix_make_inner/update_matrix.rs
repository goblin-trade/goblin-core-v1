use crate::{
    axis::occupancy::{OccupancyEnum, OccupancyMarker},
    quantities::{InnerPos, PositionV2},
    state::bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
};

pub(crate) fn update_matrix<OM>(
    resting_order_empty: bool,
    position: PositionV2,
    inner_bitmap_state: &mut InnerBitmap,
) where
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
