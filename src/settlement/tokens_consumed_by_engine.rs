use crate::{quantities::Delta, types::Address};

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TokensConsumedByEngine {
    pub index: u8,
    pub address: Address,
    pub delta: Delta,
}
