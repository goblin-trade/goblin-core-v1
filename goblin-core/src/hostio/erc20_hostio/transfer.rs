use super::call_and_check::call_and_check;
use crate::{
    goblin_error::GoblinError, hostio::abi_selector, quantities::RawAtoms, types::Address,
};

const TRANSFER_SELECTOR: [u8; 4] = abi_selector(b"transfer(address,uint256)");

pub fn transfer<const D: u8>(
    token_address: &Address,
    recipient: &Address,
    amount: &RawAtoms<D>,
) -> Result<(), GoblinError> {
    let mut calldata = [0u8; 4 + 32 * 2];

    // Function selector
    calldata[0..4].copy_from_slice(&TRANSFER_SELECTOR);

    // Encode recipient address (right-aligned in 32 bytes)
    calldata[16..36].copy_from_slice(recipient);

    // Encode amount (already 32-byte big-endian from Atoms)
    let amount_as_be_bytes: &[u8; 32] = unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) };
    calldata[36..68].copy_from_slice(amount_as_be_bytes);

    call_and_check(token_address, &calldata)
}
