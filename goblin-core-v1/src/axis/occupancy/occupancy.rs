use crate::{goblin_error::GoblinError, types::Marker};

/// Occupancy axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Occupancy;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum OccupancyEnum {
    Vacant = 0,
    Occupied = 1,
}

impl From<bool> for OccupancyEnum {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Vacant,
            true => Self::Occupied,
        }
    }
}

impl TryFrom<u8> for OccupancyEnum {
    type Error = GoblinError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OccupancyEnum::Vacant),
            1 => Ok(OccupancyEnum::Occupied),
            _ => Err(GoblinError::InvalidEnumVariant),
        }
    }
}

pub type Vacant = Marker<Occupancy, 0>;
pub type Occupied = Marker<Occupancy, 1>;
