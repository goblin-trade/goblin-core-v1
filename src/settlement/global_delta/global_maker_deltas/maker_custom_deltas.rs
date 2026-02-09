use crate::{
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    types::{CustomERC20, FixedMap},
};

///Deltas of custom tokens for various makers
pub type MakerCustomDeltas = FixedMap<MakerDeltaKey<CustomERC20>, UnsidedMakerDelta, 16>;
