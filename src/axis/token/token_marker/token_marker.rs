use crate::axis::token::{token_deltas::TokenDeltas, token_slot_store::TokenSlotStore};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker: 'static + TokenDeltas + TokenSlotStore {
    const DISCRIMINATOR: u8;
}
