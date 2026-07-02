use crate::axis::token::{token_slot_store::TokenSlotStore, HardcodedERC20};

impl TokenSlotStore for HardcodedERC20 {
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];
}
