use crate::{
    settlement::global_delta::{MakerDeltaKey, UnsidedMakerDelta},
    token::ETH,
    types::FixedMap,
};

/// Global maker deltas for ETH
pub type ETHMakerDeltas = FixedMap<MakerDeltaKey<ETH>, UnsidedMakerDelta, 16>;

impl ETHMakerDeltas {
    pub const fn zero() -> Self {
        Self {
            entries: [(
                MakerDeltaKey {
                    maker: [0u8; 20],
                    token_index: (),
                },
                UnsidedMakerDelta::zero(),
            ); 16],
            len: 0,
        }
    }
}
