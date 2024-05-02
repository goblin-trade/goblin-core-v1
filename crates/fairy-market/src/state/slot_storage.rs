//! SSTORE and SLOAD helper. .Emulates storage with a HashMap in tests
//!
//! Storage is independent of endian format. Bytes are read in the exact
//! format as they are stored.

#[cfg(test)]
use std::collections::HashMap;

#[cfg(not(test))]
use stylus_sdk::hostio;

pub struct SlotStorage {
    #[cfg(test)]
    inner: HashMap<[u8; 32], [u8; 32]>,
}

pub trait SlotActions {
    fn new() -> Self;

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]);

    fn sload(&self, key: &[u8; 32]) -> [u8; 32];
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
}

#[cfg(not(test))]
impl SlotActions for SlotStorage {
    fn new() -> Self {
        SlotStorage {}
    }

    fn sstore(&mut self, key: &[u8; 32], value: &[u8; 32]) {
        unsafe { hostio::storage_store_bytes32(key.as_ptr(), value.as_ptr()) };
    }

    fn sload(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut value = [0u8; 32];
        unsafe { hostio::storage_load_bytes32(key.as_ptr(), value.as_mut_ptr()) };

        value
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
