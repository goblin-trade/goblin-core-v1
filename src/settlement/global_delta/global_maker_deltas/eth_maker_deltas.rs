use crate::{settlement::global_delta::UnsidedMakerDelta, types::Address, utils::FixedMap};

/// Global maker deltas for ETH
pub type ETHMakerDeltas = FixedMap<Address, UnsidedMakerDelta, 16>;

impl ETHMakerDeltas {
    pub const fn new() -> Self {
        Self {
            entries: [([0u8; 20], UnsidedMakerDelta::new()); 16],
            len: 0,
        }
    }
}
