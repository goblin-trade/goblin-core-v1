use crate::{
    axis::occupancy::{OccupancyEnum, OccupancyMarker},
    quantities::{FullPosition, Position},
    state::bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
};

pub(crate) fn update_matrix<OM>(
    resting_order_empty: bool,
    position: Position,
    inner_bitmap_state: &mut InnerBitmap,
) where
    OM: OccupancyMarker,
{
    let mut inner_bitmap_updater = InnerBitmapUpdater {
        bitmap: inner_bitmap_state,
        pos: position.extract_and_convert(),
    };

    if resting_order_empty {
        inner_bitmap_updater.deactivate();
    }

    if OM::VARIANT == OccupancyEnum::Vacant {
        inner_bitmap_updater.activate();
    }
}
