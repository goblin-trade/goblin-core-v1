use crate::axis::token::{token_slot_store::TokenSlotStore, CustomERC20};

impl TokenSlotStore for CustomERC20 {
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];
}
