use crate::{
    settlement::{ConstZero, MatchedLots, MatchedLotsPair},
    types::{Address, FixedMap, Tuple},
};

pub const MAX_MAKERS: usize = 16;

/// Deltas of makers in the market namespace
///
/// This list tracks deltas generated when resting orders are matched.
pub type LocalMakerDeltas = FixedMap<Address, MatchedLotsPair, MAX_MAKERS>;

impl ConstZero for LocalMakerDeltas {
    const ZEROED: Self = Self {
        entries: [(
            [0u8; 20],
            Tuple::new(MatchedLots::zero(), MatchedLots::zero()),
        ); MAX_MAKERS],
        len: 0,
    };
}
