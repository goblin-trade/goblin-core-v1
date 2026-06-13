use crate::{
    axis::token::ETH,
    settlement::{global_delta::MakerDeltaKey, ConstZero, UnsidedTakeDeltaV2},
    types::FixedMap,
};

/// Global maker deltas for ETH
pub type ETHMakerDeltas = FixedMap<MakerDeltaKey<ETH>, UnsidedTakeDeltaV2, 16>;

impl ETHMakerDeltas {
    pub const fn zero() -> Self {
        Self {
            entries: [(
                MakerDeltaKey {
                    maker: [0u8; 20],
                    token_index: (),
                },
                UnsidedTakeDeltaV2::ZEROED,
            ); 16],
            len: 0,
        }
    }
}
