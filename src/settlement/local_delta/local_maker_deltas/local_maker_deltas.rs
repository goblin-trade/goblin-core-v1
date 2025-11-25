use crate::{settlement::local_delta::MakerDeltaPair, types::Address, utils::FixedMap};

pub const MAX_MAKERS: usize = 16;

/// Deltas of makers in the market namespace
///
/// This list tracks deltas generated when resting orders are matched.
pub type LocalMakerDeltas = FixedMap<Address, MakerDeltaPair, MAX_MAKERS>;
