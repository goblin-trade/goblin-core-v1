use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::LotSizePair,
    },
    types::{StoreReader, Tuple},
};

/// Matched atoms for a given token
#[derive(Default, PartialEq, Clone, Copy)]
pub struct MatchedAtoms<In: LegMatcher> {
    /// Atoms traded in by taker and gained by maker
    pub taker_in: In::Atoms,

    /// Atoms obtained by taker and lost by maker
    pub taker_out: In::Atoms,
}
