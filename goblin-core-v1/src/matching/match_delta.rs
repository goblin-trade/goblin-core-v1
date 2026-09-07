use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_math::LegMath, leg_quantities::LegQuantities},
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit},
};

/// Delta produced by matching against an individual resting order
pub struct MatchDelta<In: LegMatcher> {
    pub matching_lots: In::MatchingLots,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub price_in_quote_lots: QuoteLotsPerBaseUnit,
}

impl<In: LegMatcher> MatchDelta<In> {
    pub fn lots(&self) -> In::Lots {
        In::lots_taker(self.matching_lots, self.base_lot_size)
    }

    pub fn lots_opposite(&self) -> Result<<In::Opposite as LegQuantities>::Lots, GoblinError> {
        let base_lots = In::base_lots_maker(self.matching_lots, self.price_in_quote_lots);
        let matching_lots_opposite =
            In::Opposite::matching_lots_maker(base_lots, self.price_in_quote_lots)?;

        Ok(In::Opposite::lots_taker(
            matching_lots_opposite,
            self.base_lot_size,
        ))
    }
}
