use crate::{
    axis::leg::{Base, LegEnum, Quote, SamePair},
    quantities::Position,
    types::StoreReader,
};

#[derive(PartialEq, Clone, Copy)]
pub enum MakeRegion {
    Spread,
    In(LegEnum),
}

impl MakeRegion {
    pub fn new(last_positions: &SamePair<Position>, position: Position) -> Self {
        if position >= Quote::get(last_positions) {
            Self::In(LegEnum::Quote)
        } else if position <= Base::get(last_positions) {
            Self::In(LegEnum::Base)
        } else {
            Self::Spread
        }
    }
}
