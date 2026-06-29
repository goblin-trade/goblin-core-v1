use crate::{
    axis::token::token_marker::TokenMarker,
    state::{Preimage, Store},
    types::Address,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorePreimage<T: TokenMarker> {
    pub trader: Address,
    pub token: T::TokenAddress,
}

impl<T: TokenMarker> Preimage for StorePreimage<T> {
    const SLOT_DISCRIMINATOR: u8 = 2 + T::DISCRIMINATOR;
    type SlotState = Store<T>;
}
