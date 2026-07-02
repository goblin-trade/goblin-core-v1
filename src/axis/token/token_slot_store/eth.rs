use crate::axis::token::{token_marker::eth::ETHStub, token_slot_store::TokenSlotStore, ETH};

impl TokenSlotStore for ETH {
    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];
}
