use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::B256, hostio};

struct Slab {
    pub ticks: Vec<u32>,
}

// In-memory array of ticks loaded from slot. If an index is missing, read it from slot
struct Ticks {
    pub inner: Vec<u32>
}

impl Ticks {
    fn get_tick(self, index: usize) -> u32 {
        match self.inner.get(index) {
            Some(tick) => *tick,
            None => 0
        }
    }

    // fn get_tick_from_slot(&mut self, index: usize) -> u32 {

    // }
}

struct StoredTickSlot {
    pub inner: [u32; 8]
}

impl StoredTickSlot {
    fn fetch(market_index: u16, tick_slot_index: u64) -> StoredTickSlot {
        let mut key = [0u8; 32];

        key[0..8].copy_from_slice(&tick_slot_index.to_be_bytes());
        key[8..10].copy_from_slice(&market_index.to_be_bytes());

        let mut read_bytes = [0u8; 32];
        unsafe {
            hostio::storage_load_bytes32(key.as_ptr(), read_bytes.as_mut_ptr())
        };

        StoredTickSlot{ inner: [0, 0, 0, 0, 0, 0, 0, 0] }
    }
}
