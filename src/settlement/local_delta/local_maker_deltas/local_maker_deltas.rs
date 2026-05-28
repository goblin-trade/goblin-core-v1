use crate::{
    settlement::{sender_delta::pair::SidedTakeDeltaPairV2, ConstZero},
    types::{Address, FixedMap},
};

pub const MAX_MAKERS: usize = 16;

/// Deltas of makers in the market namespace
///
/// This list tracks deltas generated when resting orders are matched.
pub type LocalMakerDeltas = FixedMap<Address, SidedTakeDeltaPairV2, MAX_MAKERS>;

impl ConstZero for LocalMakerDeltas {
    const ZEROED: Self = Self {
        entries: [([0u8; 20], SidedTakeDeltaPairV2::ZEROED); MAX_MAKERS],
        len: 0,
    };
}
