use crate::{goblin_error::GoblinError, hostio, quantities::RawAtoms, require, types::Address};

// keccak256('decimals()') = 0x313ce567
const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67];

// keccak256('transfer(address,uint256)') = 0xa9059cbb
const TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

pub fn decimals(contract: &Address) -> Result<u8, GoblinError> {
    let calldata = DECIMALS_SELECTOR;
    hostio::static_call_contract(contract, calldata.as_slice())?;

    // Result is padded to 32 bytes in big endian. We need to extract a single byte.
    let decimals = hostio::read_return_data::<u8>(31);
    Ok(decimals)
}

pub fn transfer<const D: u8>(
    contract: &Address,
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

    call_and_check(contract, &calldata)
}

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

/// Perform a call and validate the response
///
/// msg.value is zero
fn call_and_check(contract: &Address, calldata: &[u8]) -> Result<(), GoblinError> {
    hostio::call_contract(contract, &calldata, &RawAtoms::<8>::ZERO)?;

    // Ensure call succeeded
    let result_byte = hostio::read_return_data::<u8>(31);
    require!(result_byte == true.into(), GoblinError::CallResultInvalid);

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
