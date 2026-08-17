use crate::axis::{occupancy::OccupancyMarker, update::UpdateMarker};

pub trait SlotSpec: Clone + Copy + PartialEq + PartialOrd {
    type Occupancy: OccupancyMarker;
    type Update: UpdateMarker;
}

impl<OM: OccupancyMarker, UM: UpdateMarker> SlotSpec for (OM, UM) {
    type Occupancy = OM;
    type Update = UM;
}
