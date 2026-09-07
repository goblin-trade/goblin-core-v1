use crate::define_axis;

define_axis! {
    pub struct Occupancy;
    enum OccupancyEnum {
        Vacant = 0,
        Occupied = 1,
    }
}
