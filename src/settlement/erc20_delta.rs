use crate::{quantities::Delta, types::Address};

#[derive(Clone, Copy)]
pub struct ERC20Delta {
    pub index: u8,
    pub address: Address,
    pub withdrawal_due: Delta,
    pub consumed_by_engine: Delta,
}
