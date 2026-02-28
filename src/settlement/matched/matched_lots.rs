use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    matching::active_iterator::coordinate::quote_pair::{self, QuotePair},
    quantities::QuantityOps,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MatchedLots<In: LegMatcher> {
    /// Input token gained by maker
    pub taker_in: In::MatchingLots,

    /// Locked output token released by maker
    pub taker_out: <In::Opposite as LegMatcher>::MatchingLots,
}

impl<In: LegMatcher> MatchedLots<In> {
    pub const fn zero() -> Self {
        Self {
            taker_in: In::MatchingLots::ZERO,
            taker_out: <In::Opposite as LegMatcher>::MatchingLots::ZERO,
        }
    }

    pub fn checked_add_v2(&mut self, quote_pair: QuotePair<In>) -> Option<()> {
        self.taker_in = self.taker_in.checked_add(quote_pair.quote)?;
        self.taker_out = self.taker_out.checked_add(quote_pair.quote_opposite)?;
        Some(())
    }

    pub fn checked_add_v3(&mut self, other: Self) -> Option<()> {
        self.taker_in = self.taker_in.checked_add(other.taker_in)?;
        self.taker_out = self.taker_out.checked_add(other.taker_out)?;
        Some(())
    }

    pub fn checked_add(
        &mut self,
        taker_in: In::MatchingLots,
        taker_out: <In::Opposite as LegMatcher>::MatchingLots,
    ) -> Result<(), GoblinError> {
        self.taker_in = self
            .taker_in
            .checked_add(taker_in)
            .ok_or(GoblinError::DeltaOverflow)?;
        self.taker_out = self
            .taker_out
            .checked_add(taker_out)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
