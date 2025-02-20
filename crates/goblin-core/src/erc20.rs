use crate::{call_contract, log_i64, log_txt, quantities::Atoms, types::Address};

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
// const TRANSFER_FROM_SELECTOR: [u8; 4] = [0, 0, 0, 0];

pub fn transfer_from(
    contract: &Address,
    sender: &Address,
    recipient: &Address,
    amount: &Atoms,
) -> u8 {
    // let mut calldata = [0u8; 4 + 32 * 3];

    // calldata[0..4].copy_from_slice(&TRANSFER_FROM_SELECTOR);

    // // 4..36: sender address
    // // 4..16 are zeroes, 16..36 holds 20 byte address
    // calldata[16..36].copy_from_slice(sender);

    // // 36..68: recipient address
    // // 36..48 are zeroes, 48..68 holds 20 byte address
    // calldata[48..68].copy_from_slice(recipient);

    // // 68..100: amount to transfer
    // // This is a 32 byte value
    // let amount_as_be_bytes: &[u8; 32] = unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) };
    // calldata[68..100].copy_from_slice(amount_as_be_bytes);

    // cast calldata "transferFrom(address,address,uint256)" 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E 0x84401cd7abbebb22acb7af2becfd9be56c30bcf1 1
    let calldata = [
        35, 184, 114, 221, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 30, 174, 125, 70, 216, 143, 8,
        252, 47, 142, 210, 127, 203, 42, 177, 131, 235, 45, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        132, 64, 28, 215, 171, 190, 187, 34, 172, 183, 175, 43, 236, 253, 155, 229, 108, 48, 188,
        241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1,
    ];
    let value = Atoms::default();

    let return_data_len: &mut usize = &mut 32;

    // Matches correct result [74, ..., 112], still error
    // Perhaps we need to copy the address?
    //
    // INFO [02-19|11:17:04.791] Submitted transaction                    hash=0xc7d8f98cc2725aad446b5e25ad05fe01f8bf0a4bf86427c27c0f330570da49b8 from=0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E nonce=8 recipient=0x7E32b54800705876d3b5cFbc7d9c226a211F7C1a value=0
    // WARN [02-19|11:18:54.461] feedOneMsg failed to send message to execEngine err="createBlock mutex held"        pos=10

    let hardcoded_token: &[u8; 20] = &[
        166, 228, 31, 253, 118, 148, 145, 164, 42, 110, 92, 228, 83, 37, 155, 147, 152, 58, 34, 239,
    ];

    unsafe {
        // 166
        let msg = b"Token byte 0 in erc20.rs";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(contract[0] as i64);

        // 239
        let msg = b"Token byte 19 in erc20.rs";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(contract[19] as i64);

        let msg = b"Token byte 0 in erc20.rs again";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(contract[0] as i64);

        // Bug- this gives
        // [184, 114, 221, 0, 0, ..., 63, 30, 174, 125, 70]
        let msg = b"Token array with index:";
        log_txt(msg.as_ptr(), msg.len());
        for i in 0..20 {
            let byte = contract[i];
            log_i64(byte as i64);
        }
    }

    // Print the entire arrays for debugging
    unsafe {
        let msg = b"Token array:";
        log_txt(msg.as_ptr(), msg.len());
        for byte in contract {
            log_i64(*byte as i64);
        }

        let msg = b"Hardcoded token array:";
        log_txt(msg.as_ptr(), msg.len());
        for byte in hardcoded_token {
            log_i64(*byte as i64);
        }
    }

    // Using `contract` or even cloning the value gives problem.
    // But why does a hardcoded token address work?
    let token = *contract;
    let token_ref = &token;

    // We get matched = false, why?
    // Byte 0 and byte 19 are the same
    let hardcoded_matches = *hardcoded_token == token;
    unsafe {
        let msg = b"Hardcoded matches";
        log_txt(msg.as_ptr(), msg.len());

        log_i64(hardcoded_matches as i64);

        if hardcoded_matches {
            let msg = b"Matched true";
            log_txt(msg.as_ptr(), msg.len());
        } else {
            let msg = b"Matched false";
            log_txt(msg.as_ptr(), msg.len());
        }
    }

    unsafe {
        call_contract(
            token_ref.as_ptr(),
            // token.as_ptr(),
            // contract.as_ptr(),
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
        let token = hex!("A6E41fFD769491a42A6e5Ce453259b93983a22EF");
        println!("token {:?}", token);
    }
}
