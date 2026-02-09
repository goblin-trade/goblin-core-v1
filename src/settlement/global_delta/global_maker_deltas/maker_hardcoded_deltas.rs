use crate::{
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    types::{FixedMap, HardcodedERC20},
};

/// Deltas of hardcoded tokens for various makers
pub type MakerHardcodedDeltas = FixedMap<MakerDeltaKey<HardcodedERC20>, UnsidedMakerDelta, 16>;
