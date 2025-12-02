use crate::{settlement::global_delta::UnsidedMakerDelta, types::Address, utils::FixedMap};

/// Global maker deltas for ETH
pub type ETHMakerDeltas = FixedMap<Address, UnsidedMakerDelta, 16>;

impl ETHMakerDeltas {
    // TODO new function
    // Remove MaybeUninit as this is stored in .bss now?
    //
    // But FixedMap is used in local delta
    //
    // Should we turn local delta into a `static mut`?
    // LocalDelta<P: PairShape> has 3 forms for each generic
    // pub const fn new() -> Self {}
}
