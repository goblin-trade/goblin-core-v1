use crate::{
    settlement::global_delta::{TokenMakerDeltaKey, UnsidedMakerDelta},
    tokens::{CustomToken, HardcodedToken},
    utils::FixedMap,
};

/// ERC20 token deltas for makers
pub struct TokenMakerDeltas {
    /// Deltas for hardcoded tokens
    pub hardcoded_token_deltas: FixedMap<TokenMakerDeltaKey<HardcodedToken>, UnsidedMakerDelta, 16>,

    /// Deltas for custom tokens
    pub custom_token_deltas: FixedMap<TokenMakerDeltaKey<CustomToken>, UnsidedMakerDelta, 16>,
}
