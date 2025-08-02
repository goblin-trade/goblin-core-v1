use crate::{goblin_error::GoblinError, hostio, quantities::RawAtoms, types::Address};

/// Transfer out native ETH to a recipient
pub fn transfer_out(recipient: &Address, amount: &RawAtoms) -> Result<(), GoblinError> {
    let calldata: [u8; 0] = [];
    hostio::call_contract(recipient, calldata.as_slice(), amount)
}
