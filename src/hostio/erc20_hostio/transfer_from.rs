use crate::{goblin_error::GoblinError, quantities::RawAtoms, types::Address};

use super::call_and_check::call_and_check;

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

pub fn transfer_from<const D: u8>(
    contract: &Address,
    sender: &Address,
    recipient: &Address,
    amount: &RawAtoms<D>,
) -> Result<(), GoblinError> {
    let mut calldata = [0u8; 4 + 32 * 3];

    calldata[0..4].copy_from_slice(&TRANSFER_FROM_SELECTOR);

    // 4..36: sender address
    calldata[16..36].copy_from_slice(sender);

    // 36..68: recipient address
    calldata[48..68].copy_from_slice(recipient);

    // 68..100: amount
    let amount_as_be_bytes: &[u8; 32] = unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) };
    calldata[68..100].copy_from_slice(amount_as_be_bytes);

    call_and_check(contract, &calldata)
}
