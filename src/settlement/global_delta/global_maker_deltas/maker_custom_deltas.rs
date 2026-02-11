use crate::{
    axis::token::CustomERC20,
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    types::FixedMap,
};

///Deltas of custom tokens for various makers
pub type MakerCustomDeltas = FixedMap<MakerDeltaKey<CustomERC20>, UnsidedMakerDelta, 16>;
