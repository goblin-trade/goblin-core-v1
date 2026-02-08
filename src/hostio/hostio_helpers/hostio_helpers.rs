///! Safe helpers for hostio interaction
use crate::{
    hostio::{hostio_helpers::buffered_call, hostio_unsafe},
    quantities::RawAtoms,
};

// Find keccak hash for a slice of bytes
//
// # Gas cost
//
// keccak is charged by the number of words. Eg. hashing a 16 bit costs the same as 256 bits.
// Padding is done by the VM so we do not have to worry.
//
// https://docs.arbitrum.io/stylus/reference/opcode-hostio-pricing#host-io-costs
//
pub fn native_keccak256(bytes: &[u8]) -> [u8; 32] {
    unsafe { buffered_call(|f| hostio_unsafe::native_keccak256(bytes.as_ptr(), bytes.len(), f)) }
}

pub fn msg_value() -> RawAtoms {
    unsafe { buffered_call(|f| hostio_unsafe::msg_value(f)) }
}

pub fn msg_sender() -> [u8; 20] {
    unsafe { buffered_call(|f| hostio_unsafe::msg_sender(f)) }
}

pub fn storage_load_bytes32<T>(key: &[u8; 32]) -> T {
    unsafe { buffered_call(|f| hostio_unsafe::storage_load_bytes32(key.as_ptr(), f)) }
}

pub fn storage_cache_bytes32<T>(key: &[u8; 32], value: &T) {
    unsafe { hostio_unsafe::storage_cache_bytes32(key.as_ptr(), value as *const T as *const u8) }
}

/// Obtain the block time of Ethereum L1 not Arbitrum L2
/// We must use the ArbOS precompile to read Arbitrum L2 block time.
/// https://docs.arbitrum.io/build-decentralized-apps/arbitrum-vs-ethereum/block-numbers-and-time#arbitrum-block-numbers
pub fn block_number() -> u32 {
    unsafe { hostio_unsafe::block_number() as u32 }
}

pub fn block_timestamp() -> u32 {
    unsafe { hostio_unsafe::block_timestamp() as u32 }
}

pub fn msg_reentrant() -> bool {
    unsafe { hostio_unsafe::msg_reentrant() }
}

pub fn storage_flush_cache(clear: bool) {
    unsafe { hostio_unsafe::storage_flush_cache(clear) }
}
