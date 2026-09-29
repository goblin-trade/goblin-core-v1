use crate::{
    axis::leg::{Base, LegEnum, Quote, SamePair, leg_coordinates::LegCoordinates},
    quantities::{FullPos, FullPosition, Ticks},
    types::StoreReader,
};

#[derive(PartialEq, Clone, Copy)]
pub enum MakeRegion {
    In(LegEnum),
    OnLastPrice(LegEnum),
    Spread,
    /// The market has never been touched: both last positions are zeroed.
    ///
    /// The quote bound of zero would make [`LegCoordinates::in_region`] true for
    /// every tick, collapsing the whole price range into the quote region. This
    /// variant lets either side open the market; the first make then seats the
    /// opener on its own side and pushes the opposite bound to the far edge of
    /// the price range.
    Unseeded,
}

impl MakeRegion {
    pub fn new(last_positions: &SamePair<FullPos>, position: FullPos) -> Self {
        let last_position_base = Base::get(last_positions);
        let last_position_quote = Quote::get(last_positions);

        if last_position_base == FullPos::ZERO && last_position_quote == FullPos::ZERO {
            return Self::Unseeded;
        }

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
