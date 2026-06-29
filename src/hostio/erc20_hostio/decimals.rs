use crate::{goblin_error::GoblinError, hostio, types::Address};

// keccak256('decimals()') = 0x313ce567
const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67];

pub fn decimals(token_address: &Address) -> Result<u8, GoblinError> {
    let calldata = DECIMALS_SELECTOR;
    hostio::static_call_contract(token_address, calldata.as_slice())?;

    // Result is padded to 32 bytes in big endian. We need to extract a single byte.
    let decimals = hostio::read_return_data::<u8>(31);
    Ok(decimals)
}
