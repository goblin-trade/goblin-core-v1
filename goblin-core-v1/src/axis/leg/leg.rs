use crate::{
    goblin_error::GoblinError,
    types::{Marker, Tuple},
};

/// Leg axis- the side of a trade
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Leg;

// TODO macro to generate axes
#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum LegEnum {
    Base = 0,
    Quote = 1,
}

impl From<bool> for LegEnum {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Base,
            true => Self::Quote,
        }
    }
}

impl TryFrom<u8> for LegEnum {
    type Error = GoblinError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LegEnum::Base),
            1 => Ok(LegEnum::Quote),
            _ => Err(GoblinError::InvalidEnumVariant),
        }
    }
}

pub type Base = Marker<Leg, { LegEnum::Base as usize }>;
pub type Quote = Marker<Leg, { LegEnum::Quote as usize }>;

pub type Pair<T0, T1> = Tuple<T0, T1, Leg>;
pub type SamePair<T> = Pair<T, T>;

impl<T0> SamePair<T0> {
    pub fn into_pair<T1>(&self) -> SamePair<T1>
    where
        T0: Clone + Copy,
        T1: From<T0>,
    {
        Pair::new(self.0.into(), self.1.into())
    }
}
