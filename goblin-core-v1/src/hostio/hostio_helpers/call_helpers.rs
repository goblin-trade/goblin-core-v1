///! Safe helpers for contract calls
use crate::{
    goblin_error::GoblinError,
    hostio::{hostio_helpers::buffered_call, hostio_unsafe},
    quantities::RawAtoms,
    require,
    types::Address,
};

pub fn call_contract<const D: u8>(
    contract: &Address,
    calldata: &[u8],
    eth: &RawAtoms<D>,
) -> Result<(), GoblinError> {
    // Use max gas to follow EVM's CALL 63/64 rule. The VM will decide how much gas to use
    let gas = u64::MAX;

    // Return data length is statically known. No need to pass the value up.
    let return_data_len = &mut 0usize;

    let result = unsafe {
        hostio_unsafe::call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            eth.0.as_ptr(),
            gas,
            return_data_len,
        )
    };

    // The return status indicates whether the call succeeded, and is 'nonzero' on failure.
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/856597767d2d24d7d93a58a970a155c5979e7903/stylus-sdk/src/hostio.rs#L161
    require!(result == 0, GoblinError::CallFail);

    Ok(())
}

pub fn static_call_contract(contract: &Address, calldata: &[u8]) -> Result<(), GoblinError> {
    let gas = u64::MAX;
    let return_data_len = &mut 0usize;

    let result = unsafe {
        hostio_unsafe::static_call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            gas,
            return_data_len,
        )
    };
    require!(result == 0, GoblinError::StaticCallFail);

    Ok(())
}

/// Read data returned from a contract call
///
/// Stylus splits calls into 2 hostios- first perform the call, then read the result
pub fn read_return_data<T>(offset: usize) -> T {
    unsafe {
        buffered_call(|f| {
            hostio_unsafe::read_return_data(f, offset, core::mem::size_of::<T>());
        })
    }
}
