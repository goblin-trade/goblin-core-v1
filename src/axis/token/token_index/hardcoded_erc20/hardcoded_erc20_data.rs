use crate::types::Address;

#[derive(Clone, Copy)]
pub struct HardcodedERC20Data {
    pub address: Address,
    pub decimals: u8,
}
