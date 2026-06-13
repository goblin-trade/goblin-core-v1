use crate::{
    axis::token::HardcodedERC20,
    settlement::{global_delta::MakerDeltaKey, UnsidedTakeDeltaV2},
    types::FixedMap,
};

/// Deltas of hardcoded tokens for various makers
pub type MakerHardcodedDeltas = FixedMap<MakerDeltaKey<HardcodedERC20>, UnsidedTakeDeltaV2, 16>;
