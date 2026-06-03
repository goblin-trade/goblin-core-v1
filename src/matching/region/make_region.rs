use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, LegEnum, Quote, SamePair},
    quantities::{Position, Ticks},
    types::StoreReader,
};

#[derive(PartialEq, Clone, Copy)]
pub enum MakeRegion {
    In(LegEnum),
    OnLastPrice(LegEnum),
    Spread,
}

impl MakeRegion {
    pub fn new(last_positions: &SamePair<Position>, position: Position) -> Self {
        let last_position_base = Base::get(last_positions);
        let last_position_quote = Quote::get(last_positions);

        let price = Ticks::from(position);
        let last_price_base = Ticks::from(last_position_base);
        let last_price_quote = Ticks::from(last_position_quote);

        if Quote::in_region(last_position_quote, position) {
            Self::In(LegEnum::Quote)
        } else if Base::in_region(last_position_base, position) {
            Self::In(LegEnum::Base)
        } else if price == last_price_base {
            Self::OnLastPrice(LegEnum::Base)
        } else if price == last_price_quote {
            Self::OnLastPrice(LegEnum::Quote)
        } else {
            Self::Spread
        }
    }
}
