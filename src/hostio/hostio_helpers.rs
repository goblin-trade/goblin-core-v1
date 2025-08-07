use core::u64;

use crate::{
    goblin_error::GoblinError, input_processor::ArgsBuffer, quantities::RawAtoms, require,
    types::Address,
};

use super::{hostio_unsafe, HostioBuffer};

pub fn read_args() -> HostioBuffer<ArgsBuffer> {
    unsafe { HostioBuffer::<ArgsBuffer>::new(|ptr| hostio_unsafe::read_args(ptr)) }
}

pub fn msg_sender() -> HostioBuffer<Address> {
    unsafe { HostioBuffer::<Address>::new(|ptr| hostio_unsafe::msg_sender(ptr)) }
}

pub fn msg_value() -> HostioBuffer<RawAtoms> {
    unsafe { HostioBuffer::<RawAtoms>::new(|f| hostio_unsafe::msg_value(f)) }
}

pub fn native_keccak256(bytes: &[u8]) -> HostioBuffer<[u8; 32]> {
    unsafe {
        HostioBuffer::<[u8; 32]>::new(|f| {
            hostio_unsafe::native_keccak256(bytes.as_ptr(), bytes.len(), f)
        })
    }
}

pub fn storage_load_bytes32<T>(key: &[u8; 32]) -> HostioBuffer<T> {
    unsafe { HostioBuffer::<T>::new(|f| hostio_unsafe::storage_load_bytes32(key.as_ptr(), f)) }
}

pub fn storage_cache_bytes32<T>(key: &[u8; 32], value: &T) {
    unsafe { hostio_unsafe::storage_cache_bytes32(key.as_ptr(), value as *const T as *const u8) }
}

pub fn block_number() -> u32 {
    unsafe { hostio_unsafe::block_number() as u32 }
}

pub fn block_timestamp() -> u32 {
    unsafe { hostio_unsafe::block_timestamp() as u32 }
}

pub fn order_not_expired(is_block_number: bool, expiry_value: u32) -> bool {
    match is_block_number {
        true => block_number() <= expiry_value,
        false => block_timestamp() <= expiry_value,
    }
}

pub fn msg_reentrant() -> bool {
    unsafe { hostio_unsafe::msg_reentrant() }
}

pub fn storage_flush_cache(clear: bool) {
    unsafe { hostio_unsafe::storage_flush_cache(clear) }
}

pub fn call_contract(
    contract: &Address,
    calldata: &[u8],
    eth: &RawAtoms,
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

pub fn read_return_data<T>(offset: usize) -> HostioBuffer<T> {
    unsafe {
        HostioBuffer::<T>::new(|f| {
            hostio_unsafe::read_return_data(f, offset, core::mem::size_of::<T>());
        })
    }
}
