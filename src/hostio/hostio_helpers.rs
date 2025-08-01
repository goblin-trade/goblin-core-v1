use crate::{input_processor::ArgsBuffer, quantities::RawAtoms, types::Address};

use super::{hostio, HostioBuffer};

// TODO shorten names, move unsafe block inside here
// The raw hostio module should be private and not directly accessed. Unsafe code scattered
// randomly looks ugly

pub unsafe fn hostio_read_args() -> HostioBuffer<ArgsBuffer> {
    HostioBuffer::<ArgsBuffer>::new(|ptr| hostio::read_args(ptr))
}

pub unsafe fn hostio_msg_sender() -> HostioBuffer<Address> {
    HostioBuffer::<Address>::new(|ptr| hostio::msg_sender(ptr))
}

pub unsafe fn hostio_msg_value() -> HostioBuffer<RawAtoms> {
    HostioBuffer::<RawAtoms>::new(|f| hostio::msg_value(f))
}

pub unsafe fn hostio_native_keccak256(bytes: &[u8]) -> HostioBuffer<[u8; 32]> {
    HostioBuffer::<[u8; 32]>::new(|f| hostio::native_keccak256(bytes.as_ptr(), bytes.len(), f))
}

pub unsafe fn hostio_storage_load_bytes32<T>(key: &[u8; 32]) -> HostioBuffer<T> {
    HostioBuffer::<T>::new(|f| hostio::storage_load_bytes32(key.as_ptr(), f))
}

pub unsafe fn hostio_storage_cache_bytes32<T>(key: &[u8; 32], value: &T) {
    hostio::storage_cache_bytes32(key.as_ptr(), value as *const T as *const u8)
}

pub fn hostio_block_number() -> u64 {
    unsafe { hostio::block_number() }
}

pub fn hostio_block_timestamp() -> u64 {
    unsafe { hostio::block_timestamp() }
}
