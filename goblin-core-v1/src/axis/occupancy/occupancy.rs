use crate::types::Marker;

/// Occupancy axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Occupancy;

pub type Vacant = Marker<Occupancy, 0>;
pub type Occupied = Marker<Occupancy, 1>;
