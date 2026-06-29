use crate::{goblin_error::GoblinError, hostio, quantities::RawAtoms, require, types::Address};

/// Perform a call and validate the response
///
/// msg.value is zero
pub fn call_and_check(contract: &Address, calldata: &[u8]) -> Result<(), GoblinError> {
    hostio::call_contract(contract, &calldata, &RawAtoms::<8>::ZERO)?;

    // Ensure call succeeded
    let result_byte = hostio::read_return_data::<u8>(31);
    require!(result_byte == true.into(), GoblinError::CallResultInvalid);

    Ok(())
}
