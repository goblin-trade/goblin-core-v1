use crate::{
    settlement::global_delta::{ERC20MakerDeltaKey, UnsidedMakerDelta},
    token::{CustomToken, HardcodedToken},
    types::{ERC20Pair, FixedMap},
};

/// ERC20 token deltas for makers
pub type ERC20MakerDeltas = ERC20Pair<MakerHardcodedDeltas, MakerCustomDeltas>;

impl ERC20MakerDeltas {
    pub const fn zero() -> Self {
        Self::new(
            FixedMap {
                entries: [(
                    ERC20MakerDeltaKey::<HardcodedToken>::zero(),
                    UnsidedMakerDelta::zero(),
                ); 16],
                len: 0,
            },
            FixedMap {
                entries: [(
                    ERC20MakerDeltaKey::<CustomToken>::zero(),
                    UnsidedMakerDelta::zero(),
                ); 16],
                len: 0,
            },
        )
    }
}

/// Deltas of hardcoded tokens for various makers
pub type MakerHardcodedDeltas = FixedMap<ERC20MakerDeltaKey<HardcodedToken>, UnsidedMakerDelta, 16>;

///Deltas of custom tokens for various makers
pub type MakerCustomDeltas = FixedMap<ERC20MakerDeltaKey<CustomToken>, UnsidedMakerDelta, 16>;
