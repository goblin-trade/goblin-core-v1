use crate::{axis::leg::leg_matcher::LegMatcher, quantities::QuantityOps};

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

    pub fn checked_add(&mut self, rhs: Self) -> Option<()> {
        self.taker_in = self.taker_in.checked_add(rhs.taker_in)?;
        self.taker_out = self.taker_out.checked_add(rhs.taker_out)?;

        Some(())
    }
}
