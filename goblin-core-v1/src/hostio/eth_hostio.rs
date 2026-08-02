use crate::{goblin_error::GoblinError, hostio, quantities::RawAtoms, types::Address};

/// Transfer out native ETH to a recipient
pub fn transfer_out<const D: u8>(
    recipient: &Address,
    amount: &RawAtoms<D>,
) -> Result<(), GoblinError> {
    let calldata: [u8; 0] = [];
    hostio::call_contract(recipient, calldata.as_slice(), amount)
}
