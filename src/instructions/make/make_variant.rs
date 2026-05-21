use crate::axis::{leg::LegEnum, update::UpdateEnum};

pub enum MakeVariant {
    Update(UpdateEnum),
    Open(LegEnum),
    Limit(LegEnum),
}

impl From<u64> for MakeVariant {
    fn from(value: u64) -> Self {
        match value & 0b11 {
            0 => MakeVariant::Update(UpdateEnum::Decrease),
            1 => MakeVariant::Update(UpdateEnum::Decrease),
            2 => MakeVariant::Open(LegEnum::Base),
            3 => MakeVariant::Open(LegEnum::Quote),
            4 => MakeVariant::Limit(LegEnum::Base),
            5 => MakeVariant::Limit(LegEnum::Quote),
            _ => unreachable!(),
        }
    }
}
