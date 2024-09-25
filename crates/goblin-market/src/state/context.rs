//! Context to EVM opcodes (SSTORE and SLOAD) and variables (block time, block number).
//! Emulated in tests with a HashMap for storage and hardcoded values for variables.
//!
//! Storage is independent of endian format. Bytes are read in the exact
//! format as they are stored.

#[cfg(test)]
use std::collections::HashMap;

#[cfg(not(test))]
use stylus_sdk::hostio;

pub const LIST_KEY_SEED: u8 = 0;
pub const BITMAP_GROUP_SEED: u8 = 1;
pub const RESTING_ORDER_KEY_SEED: u8 = 2;
pub const TRADER_STATE_KEY_SEED: u8 = 3;
pub const MARKET_STATE_KEY_SEED: u8 = 4;

pub struct ArbContext {
    #[cfg(test)]
    inner: HashMap<[u8; 32], [u8; 32]>,

    #[cfg(test)]
    block_number: u64,

    #[cfg(test)]
    block_timestamp: u64,
}

pub trait ContextActions {
    fn new() -> Self;

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]);

    fn sload(&self, key: &[u8; 32]) -> [u8; 32];

    fn storage_flush_cache(clear: bool);

    /// Current block number
    fn block_number(&self) -> u64;

    /// Current block epoch time in seconds
    fn block_timestamp(&self) -> u64;
}

impl ArbContext {
    #[cfg(test)]
    pub fn new_with_block_details(block_number: u64, block_timestamp: u64) -> Self {
        ArbContext {
            inner: HashMap::new(),
            block_number,
            block_timestamp,
        }
    }
}

#[cfg(test)]
impl ContextActions for ArbContext {
    fn new() -> Self {
        ArbContext {
            inner: HashMap::new(),
            block_number: 0,
            block_timestamp: 0,
        }
    }

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]) {
        self.inner.insert(*key, *value);
    }

    fn sload(&self, key: &[u8; 32]) -> [u8; 32] {
        *self.inner.get(key).unwrap_or(&[0u8; 32])
    }

    fn storage_flush_cache(_clear: bool) {}

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn block_timestamp(&self) -> u64 {
        self.block_timestamp
    }
}

#[cfg(not(test))]
impl ContextActions for ArbContext {
    fn new() -> Self {
        ArbContext {}
    }

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]) {
        unsafe { hostio::storage_cache_bytes32(key.as_ptr(), value.as_ptr()) };
    }

    // important: call hostio::storage_flush_cache() before exiting or calling other contracts
    fn sload(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut value = [0u8; 32];
        unsafe { hostio::storage_load_bytes32(key.as_ptr(), value.as_mut_ptr()) };

        value
    }

    fn storage_flush_cache(clear: bool) {
        unsafe { hostio::storage_flush_cache(clear) };
    }

    fn block_number(&self) -> u64 {
        unsafe { hostio::block_number() }
    }

    fn block_timestamp(&self) -> u64 {
        unsafe { hostio::block_timestamp() }
    }
}

#[cfg(not(test))]
impl Drop for ArbContext {
    fn drop(&mut self) {
        // Write cache to slot
        unsafe { hostio::storage_flush_cache(false) };
    }
}

pub trait SlotKey {
    fn get_key(&self) -> [u8; 32];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_and_read() {
        let mut ctx = ArbContext::new();

        let key = &[0u8; 32];
        let value: [u8; 32] = [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        ];

        assert_eq!(ctx.sload(key), [0u8; 32]);

        ctx.sstore(key, &value);
        assert_eq!(ctx.sload(key), value);
    }
}
