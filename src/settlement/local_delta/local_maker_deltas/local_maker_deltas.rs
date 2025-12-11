use crate::{
    settlement::local_delta::MakerDeltaPair,
    types::{Address, FixedMap},
};

pub const MAX_MAKERS: usize = 16;

/// Deltas of makers in the market namespace
///
/// This list tracks deltas generated when resting orders are matched.
pub type LocalMakerDeltas = FixedMap<Address, MakerDeltaPair, MAX_MAKERS>;

impl LocalMakerDeltas {
    pub const fn zero() -> Self {
        Self {
            entries: [([0u8; 20], MakerDeltaPair::zero()); MAX_MAKERS],
            len: 0,
        }
    }
}
