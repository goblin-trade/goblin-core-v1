use crate::{
    settlement::global_delta::{ERC20MakerDeltaKey, UnsidedMakerDelta},
    token::{CustomToken, HardcodedToken},
    types::FixedMap,
};

/// ERC20 token deltas for makers
pub struct ERC20MakerDeltas {
    /// Deltas for hardcoded tokens
    pub hardcoded_token_deltas: FixedMap<ERC20MakerDeltaKey<HardcodedToken>, UnsidedMakerDelta, 16>,

    /// Deltas for custom tokens
    pub custom_token_deltas: FixedMap<ERC20MakerDeltaKey<CustomToken>, UnsidedMakerDelta, 16>,
}

impl ERC20MakerDeltas {
    pub const fn zero() -> Self {
        Self {
            hardcoded_token_deltas: FixedMap {
                entries: [(
                    ERC20MakerDeltaKey::<HardcodedToken>::zero(),
                    UnsidedMakerDelta::zero(),
                ); 16],
                len: 0,
            },
            custom_token_deltas: FixedMap {
                entries: [(
                    ERC20MakerDeltaKey::<CustomToken>::zero(),
                    UnsidedMakerDelta::zero(),
                ); 16],
                len: 0,
            },
        }
    }
}
