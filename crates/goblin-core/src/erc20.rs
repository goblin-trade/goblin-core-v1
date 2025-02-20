use crate::{call_contract, quantities::Atoms, types::Address};

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
// const TRANSFER_FROM_SELECTOR: [u8; 4] = [0, 0, 0, 0];

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
    let return_data_len: &mut usize = &mut 32;

    unsafe {
        call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            value.0.as_ptr() as *const u8, // Zero value
            200_000, // 200k gas. We need to explicitly specify gas else, tx fails
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

    #[test]
    fn test_encode_as_arr() {
        // cast calldata "transferFrom(address,address,uint256)" 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E 0x84401cd7abbebb22acb7af2becfd9be56c30bcf1 1
        let calldata = hex!("23b872dd0000000000000000000000003f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e00000000000000000000000084401cd7abbebb22acb7af2becfd9be56c30bcf10000000000000000000000000000000000000000000000000000000000000001");

        println!("calldata {:?}", calldata);
    }

    #[test]
    fn test_get_token_as_arr() {
        let token = hex!("F5FfD11A55AFD39377411Ab9856474D2a7Cb697e");
        println!("token {:?}", token);
    }

    #[test]
    fn test_get_contract_as_arr() {
        let token = hex!("a6e41ffd769491a42a6e5ce453259b93983a22ef");
        println!("token {:?}", token);
    }
}
