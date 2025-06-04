use crate::{
    input_processor::PayloadBuffer, quantities::RawAtoms, state::SlotKeyV2, types::Address,
};

use super::{hostio, HostioBuffer};

pub unsafe fn hostio_read_args() -> HostioBuffer<PayloadBuffer> {
    HostioBuffer::<PayloadBuffer>::new(|ptr| hostio::read_args(ptr))
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
