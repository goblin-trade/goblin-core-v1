use crate::{
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    token::CustomERC20,
    types::FixedMap,
};

///Deltas of custom tokens for various makers
pub type MakerCustomDeltas = FixedMap<MakerDeltaKey<CustomERC20>, UnsidedMakerDelta, 16>;
