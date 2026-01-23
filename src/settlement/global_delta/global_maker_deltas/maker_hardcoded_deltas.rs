use crate::{
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    token::HardcodedERC20,
    types::FixedMap,
};

/// Deltas of hardcoded tokens for various makers
pub type MakerHardcodedDeltas = FixedMap<MakerDeltaKey<HardcodedERC20>, UnsidedMakerDelta, 16>;
