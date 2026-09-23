use goblin_macros::define_axis;

#[define_axis]
pub enum OccupancyEnum {
    Vacant = 0,
    Occupied = 1,
}
