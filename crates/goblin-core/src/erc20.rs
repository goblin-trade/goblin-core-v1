use crate::{call_contract, quantities::Atoms, types::Address};

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

pub fn transfer_from(
    contract: &Address,
    sender: &Address,
    recipient: &Address,
    amount: &Atoms,
) -> u8 {
    let mut calldata = [0u8; 4 + 32 * 3];

    calldata[0..4].copy_from_slice(&TRANSFER_FROM_SELECTOR);

    // 4..36: sender address
    // 4..16 are zeroes, 16..36 holds 20 byte address
    calldata[16..36].copy_from_slice(sender);

    // 36..68: recipient address
    // 36..48 are zeroes, 48..68 holds 20 byte address
    calldata[48..68].copy_from_slice(recipient);

    // 68..100: amount to transfer
    // This is a 32 byte value
    let amount_as_be_bytes: &[u8; 32] = unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) };
    calldata[68..100].copy_from_slice(amount_as_be_bytes);

    let value = Atoms::default();

    let return_data_len: &mut usize = &mut 0;

    unsafe {
        call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            value.0.as_ptr() as *const u8, // Zero value
            0,                             // Zero gas
            return_data_len as *mut usize,
        )
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    #[test]
    fn test_amount_encoding() {
        let amount = hex!("00000001");
        println!("amount {:?}", amount);
    }
}
