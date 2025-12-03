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

impl TokenMakerDeltas {
    pub const fn new() -> Self {
        Self {
            hardcoded_token_deltas: FixedMap {
                entries: [(
                    TokenMakerDeltaKey::<HardcodedToken>::new(),
                    UnsidedMakerDelta::new(),
                ); 16],
                len: 0,
            },
            custom_token_deltas: FixedMap {
                entries: [(
                    TokenMakerDeltaKey::<CustomToken>::new(),
                    UnsidedMakerDelta::new(),
                ); 16],
                len: 0,
            },
        }
    }
}
