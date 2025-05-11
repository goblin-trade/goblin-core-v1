use core::mem::MaybeUninit;

use crate::{
    call, goblin_error::GoblinError, hostio, quantities::RawAtoms, require, types::Address,
};

// keccak256('decimals()') = 0x313ce567
const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67];

// keccak256('transfer(address,uint256)') = 0xa9059cbb
const TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

pub fn decimals(contract: &Address) -> Result<u8, GoblinError> {
    let calldata = DECIMALS_SELECTOR;
    let return_data_len: &mut usize = &mut 0;

    let call_result = unsafe {
        call::static_call(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            u64::MAX,
            return_data_len,
        )
    };

    require!(call_result == 0, GoblinError::DecimalReadFail);

    // Result is padded to 32 bytes in big endian. We need to extract a single byte.
    let mut decimals_maybe = MaybeUninit::<u8>::uninit();
    let decimals = unsafe {
        hostio::read_return_data(decimals_maybe.as_mut_ptr(), 31, 1);
        decimals_maybe.assume_init_ref()
    };

    Ok(*decimals)
}

pub fn transfer(
    contract: &Address,
    recipient: &Address,
    amount: &RawAtoms,
) -> Result<(), GoblinError> {
    let mut calldata = [0u8; 4 + 32 * 2];

    // Function selector
    calldata[0..4].copy_from_slice(&TRANSFER_SELECTOR);

    // Encode recipient address (right-aligned in 32 bytes)
    calldata[16..36].copy_from_slice(recipient);

    // Encode amount (already 32-byte big-endian from Atoms)
    let amount_as_be_bytes: &[u8; 32] = unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) };
    calldata[36..68].copy_from_slice(amount_as_be_bytes);

    let zero_value = RawAtoms::default(); // Sending tokens, not ETH
    let return_data_len: &mut usize = &mut 0;

    let call_result = unsafe {
        call::clear_cache_and_call(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            zero_value.0.as_ptr() as *const u8,
            u64::MAX,
            return_data_len,
        )
    };
    require!(call_result == 0, GoblinError::CallFail);

    // Check if the return value is false
    let mut result_byte_maybe = MaybeUninit::<u8>::uninit();
    let result_byte = unsafe {
        hostio::read_return_data(result_byte_maybe.as_mut_ptr(), 31, 1);
        result_byte_maybe.assume_init_ref()
    };

    require!(*result_byte == true.into(), GoblinError::CallResultInvalid);

    Ok(())
}

pub fn transfer_from(
    contract: &Address,
    sender: &Address,
    recipient: &Address,
    amount: &RawAtoms,
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

    let zero_value = RawAtoms::default();
    let return_data_len: &mut usize = &mut 0;

    let call_result = unsafe {
        call::clear_cache_and_call(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            zero_value.0.as_ptr() as *const u8,
            // Use max gas to follow EVM's CALL 63/64 rule. The VM will decide how much gas to use
            u64::MAX,
            return_data_len,
        )
    };

    // If the call itself failed, treat as error.
    require!(call_result == 0, GoblinError::CallFail);

    // If the contract returned `false`, treat as error.
    let mut result_byte_maybe = MaybeUninit::<u8>::uninit();
    let result_byte = unsafe {
        hostio::read_return_data(result_byte_maybe.as_mut_ptr(), 31, 1);
        result_byte_maybe.assume_init_ref()
    };

    require!(*result_byte == true.into(), GoblinError::CallResultInvalid);

    Ok(())
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
