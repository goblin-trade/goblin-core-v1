use crate::axis::{leg::LegEnum, occupancy::OccupancyEnum, update::UpdateEnum};

pub struct MakeVariantV2 {
    pub occupancy: OccupancyEnum,
    pub inner_enum_raw: u8,
}

impl From<u64> for MakeVariantV2 {
    fn from(value: u64) -> Self {
        let occupancy = match value & 0b1 {
            0 => OccupancyEnum::Vacant,
            1 => OccupancyEnum::Occupied,
            _ => unreachable!(),
        };
        let inner_enum_raw = (value & 0b10) as u8;

        Self {
            occupancy,
            inner_enum_raw,
        }
    }
}

pub enum MakeVariant {
    Vacant(LegEnum),
    Occupied(UpdateEnum),
}

impl From<u64> for MakeVariant {
    fn from(value: u64) -> Self {
        match value & 0b11 {
            0 => MakeVariant::Vacant(LegEnum::Base),
            1 => MakeVariant::Vacant(LegEnum::Quote),
            2 => MakeVariant::Occupied(UpdateEnum::Increase),
            3 => MakeVariant::Occupied(UpdateEnum::Decrease),
            _ => unreachable!(),
        }
    }
}
