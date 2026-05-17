use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, LegEnum, Quote, SamePair},
    quantities::{SafePosition, POS_2},
    types::StoreReader,
};

#[derive(PartialEq, Clone, Copy)]
pub enum MakeRegion {
    Spread,
    In(LegEnum),
}

impl MakeRegion {
    pub fn new(
        last_positions: &SamePair<SafePosition<POS_2>>,
        position: SafePosition<POS_2>,
    ) -> Self {
        let last_position_quote = Quote::get(last_positions);
        let last_position_base = Base::get(last_positions);

        if Quote::in_region(last_position_quote, position) {
            Self::In(LegEnum::Quote)
        } else if Base::in_region(last_position_base, position) {
            Self::In(LegEnum::Base)
        } else {
            Self::Spread
        }
    }
}
