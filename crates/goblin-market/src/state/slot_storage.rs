//! SSTORE and SLOAD helper. .Emulates storage with a HashMap in tests
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

// #[derive(Clone)]
pub struct SlotStorage {
    #[cfg(test)]
    inner: HashMap<[u8; 32], [u8; 32]>,
}

pub trait SlotActions {
    fn new() -> Self;

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]);

    fn sload(&self, key: &[u8; 32]) -> [u8; 32];

    fn storage_flush_cache(clear: bool);
}

#[cfg(test)]
impl SlotActions for SlotStorage {
    fn new() -> Self {
        SlotStorage {
            inner: HashMap::new(),
        }
    }

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]) {
        self.inner.insert(*key, *value);
    }

    fn sload(&self, key: &[u8; 32]) -> [u8; 32] {
        *self.inner.get(key).unwrap_or(&[0u8; 32])
    }

    fn storage_flush_cache(_clear: bool) {}
}

#[cfg(not(test))]
impl SlotActions for SlotStorage {
    fn new() -> Self {
        SlotStorage {}
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
}

#[cfg(not(test))]
impl Drop for SlotStorage {
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
        let mut slot_storage = SlotStorage::new();

        let key = &[0u8; 32];
        let value: [u8; 32] = [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        ];

        assert_eq!(slot_storage.sload(key), [0u8; 32]);

        slot_storage.sstore(key, &value);
        assert_eq!(slot_storage.sload(key), value);
    }
}
