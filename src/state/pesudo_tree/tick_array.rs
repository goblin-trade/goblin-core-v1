use crate::state::slot_storage::{SlotActions, SlotKey, SlotStorage};
use alloc::vec::Vec;

pub struct TickArrayKey {
    // The market index
    pub market_index: u16,

    // Array beginning from 0
    pub tick_slot_index: u32,
}

impl SlotKey for TickArrayKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0..4].copy_from_slice(&self.tick_slot_index.to_le_bytes());
        key[4..6].copy_from_slice(&self.market_index.to_le_bytes());

        key
    }
}

pub struct TickArray {
    /// Tick array belongs to which market
    pub market_index: u16,

    /// In-memory cache of tick array slots
    pub cached_tick_bytes: Vec<u8>,

    /// cached_tick_bytes from this index are pending an update
    pub start_index_pending_update: Option<usize>,

    /// Whether all ticks have been read from slot
    pub end_reached: bool,

    /// Wether ticks are in ascending or descending order
    pub ascending: bool,

    pub slot_storage: SlotStorage,
}

impl Iterator for TickArray {
    type Item = [u8; 32];

    /// Read and return the next tick array slot from SLOAD
    /// The read slot is cached
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.cached_tick_bytes.len();
        let key = self.get_tick_key(index as u32);

        let item: [u8; 32] = self.slot_storage.sload(&key);

        if item == [0u8; 32] {
            self.end_reached = true;

            None
        } else {
            self.cached_tick_bytes.extend(item);
            Some(item)
        }
    }
}

impl TickArray {
    pub fn new(market_index: u16, slot_storage: SlotStorage, ascending: bool) -> Self {
        TickArray {
            market_index,
            cached_tick_bytes: Vec::<u8>::new(),
            start_index_pending_update: None,
            end_reached: false,
            ascending,
            slot_storage,
        }
    }

    pub fn load_ticks(&mut self) {
        loop {
            match self.next() {
                Some(_) => {}
                None => {
                    break;
                }
            }
        }
    }

    pub fn get_tick_key(&self, tick_slot_index: u32) -> [u8; 32] {
        let tick_array_key = TickArrayKey {
            market_index: self.market_index.into(),
            // Cast will not overflow. There can be max 2^64 - 1 ticks. Number of tick slots is smaller.
            tick_slot_index,
        };

        tick_array_key.get_key()
    }

    /// Convert raw bits into a vector of ticks
    /// Each tick is of size u32, interpreted from 3 bytes of the bits vector
    pub fn ticks(&self, start_index: usize) -> Vec<u32> {
        let mut ticks: Vec<u32> = Vec::<u32>::new();

        // This skips the last 1-2 bytes in the end that cannot form a complete tick
        let end_index = (self.cached_tick_bytes.len() / 3) * 3;

        for i in (start_index..end_index).step_by(3) {
            let byte_0 = *self.cached_tick_bytes.get(i).unwrap();
            let byte_1 = *self.cached_tick_bytes.get(i + 1).unwrap();
            let byte_2 = *self.cached_tick_bytes.get(i + 2).unwrap();

            let tick = u32::from_le_bytes([byte_0, byte_1, byte_2, 0]);

            ticks.push(tick);
        }

        ticks
    }

    /// Insert a new tick into the sorted ticks array.
    /// All written ticks must be read first before new items can be added
    pub fn insert(&mut self, tick: u32) {
        assert!(self.end_reached);

        let bytes_insert_index = if self.cached_tick_bytes.len() == 0 {
            0
        } else {
            let mut ticks = self.ticks(0);

            // TODO fix insertion for descending
            if !self.ascending {
                ticks.reverse();
            };
            // Error variant returns insertion index if the element is not present
            let mut tick_insert_index = ticks.binary_search(&tick).unwrap_err();

            if !self.ascending {
                tick_insert_index = ticks.len() - tick_insert_index
            }
            tick_insert_index * 3
        };

        // Obtain tick bytes
        let tick_bytes = tick.to_le_bytes();

        self.cached_tick_bytes
            .insert(bytes_insert_index, tick_bytes[0]);
        self.cached_tick_bytes
            .insert(bytes_insert_index + 1, tick_bytes[1]);
        self.cached_tick_bytes
            .insert(bytes_insert_index + 2, tick_bytes[2]);

        self.set_start_index_pending_update(bytes_insert_index);
    }

    /// Try to remove a tick from the tick array
    /// Three zeroes are added for the removed tick such that size of cached_tick_bytes does not change
    pub fn remove(&mut self, tick: u32) {
        assert!(self.end_reached);

        let ticks = self.ticks(0);

        // Ok variant returns insertion index if the element is present
        let tick_remove_index = ticks.binary_search(&tick).unwrap();
        let bytes_remove_index = tick_remove_index * 3;

        // Remove 3 bytes
        self.cached_tick_bytes.remove(bytes_remove_index);
        self.cached_tick_bytes.remove(bytes_remove_index);
        self.cached_tick_bytes.remove(bytes_remove_index);

        // Add empty space at the end
        self.cached_tick_bytes.extend(&[0u8, 0u8, 0u8]);

        self.set_start_index_pending_update(bytes_remove_index);
    }

    /// Try to set the index from where bytes need to be written to slot
    pub fn set_start_index_pending_update(&mut self, new_index: usize) {
        match self.start_index_pending_update {
            None => self.start_index_pending_update = Some(new_index),
            Some(old_index) => {
                // Min value
                if new_index < old_index {
                    self.start_index_pending_update = Some(new_index);
                }
            }
        }
    }

    /// Save the diff to slot
    pub fn flush(&mut self) {
        assert!(self.end_reached);
        assert!(self.start_index_pending_update.is_some());

        let slot_index_to_update_from = self.start_index_pending_update.unwrap() / 32;
        let end_index = self.cached_tick_bytes.len();

        for i in (slot_index_to_update_from..end_index).step_by(32) {
            let end_index = (i + 32).min(end_index);
            let chunk = &self.cached_tick_bytes[i..end_index];

            let mut slot = [0u8; 32];
            for (j, &byte) in chunk.iter().enumerate() {
                slot[j] = byte;
            }

            let slot_key = self.get_tick_key((i as u32) / 32);

            self.slot_storage.sstore(&slot_key, &slot);
        }

        // Clear index after saving to slot
        self.start_index_pending_update = None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_and_removal_on_empty_slot() {
        let slot_storage = SlotStorage::new();
        let mut tick_array = TickArray::new(0, slot_storage, true);

        assert!(!tick_array.end_reached);
        assert!(tick_array.start_index_pending_update.is_none());
        assert!(tick_array.ticks(0) == vec![]);
        assert!(tick_array.cached_tick_bytes == vec![]);

        // Call next() when slots are empty
        let read_slot = tick_array.next();
        assert!(read_slot.is_none());
        assert!(tick_array.end_reached);
        assert!(tick_array.ticks(0) == vec![]);
        assert!(tick_array.cached_tick_bytes == vec![]);

        tick_array.insert(1);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0]);

        tick_array.insert(3);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1, 3]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0, 3, 0, 0]);

        tick_array.insert(2);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1, 2, 3]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0, 2, 0, 0, 3, 0, 0]);

        tick_array.remove(2);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1, 3, 0]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0, 3, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_insertion_and_removal_on_empty_slot_in_descending_order() {
        let slot_storage = SlotStorage::new();
        let mut tick_array = TickArray::new(0, slot_storage, false);

        assert!(!tick_array.end_reached);
        assert!(tick_array.start_index_pending_update.is_none());
        assert!(tick_array.ticks(0) == vec![]);
        assert!(tick_array.cached_tick_bytes == vec![]);

        // Call next() when slots are empty
        let read_slot = tick_array.next();
        assert!(read_slot.is_none());
        assert!(tick_array.end_reached);
        assert!(tick_array.ticks(0) == vec![]);
        assert!(tick_array.cached_tick_bytes == vec![]);

        tick_array.insert(1);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0]);

        tick_array.insert(3);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);

        assert!(tick_array.ticks(0) == vec![3, 1]);
        assert!(tick_array.cached_tick_bytes == vec![3, 0, 0, 1, 0, 0]);

        tick_array.insert(2);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![3, 2, 1]);
        assert!(tick_array.cached_tick_bytes == vec![3, 0, 0, 2, 0, 0, 1, 0, 0]);

        tick_array.remove(2);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![3, 1, 0]);
        assert!(tick_array.cached_tick_bytes == vec![3, 0, 0, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_flush() {
        let slot_storage = SlotStorage::new();
        let market_index = 0;

        let mut tick_array = TickArray::new(market_index, slot_storage, true);
        tick_array.load_ticks();

        tick_array.insert(1);
        assert!(tick_array.start_index_pending_update.unwrap() == 0);
        assert!(tick_array.ticks(0) == vec![1]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0]);

        tick_array.flush();
        assert!(tick_array.start_index_pending_update.is_none());
        assert!(tick_array.ticks(0) == vec![1]);
        assert!(tick_array.cached_tick_bytes == vec![1, 0, 0]);

        let key_0 = TickArrayKey {
            market_index,
            tick_slot_index: 0,
        }
        .get_key();
        let key_1 = TickArrayKey {
            market_index,
            tick_slot_index: 1,
        }
        .get_key();

        let mut slot_0 = tick_array.slot_storage.sload(&key_0);
        assert_eq!(
            slot_0,
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );

        // case 1- ticks remain in same slot
        for i in 2..=10 {
            tick_array.insert(i);
        }
        assert!(tick_array.start_index_pending_update.unwrap() == 3);
        tick_array.flush();

        let expected_ticks: Vec<u32> = (1..=10).collect();
        assert!(tick_array.ticks(0) == expected_ticks);

        let mut expected_bytes: Vec<u8> = vec![
            1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0,
            0,
        ];

        assert_eq!(tick_array.cached_tick_bytes, expected_bytes);

        slot_0 = tick_array.slot_storage.sload(&key_0);
        let mut expected_slot_0: [u8; 32] = [
            1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0,
            0, 0, 0,
        ];
        assert_eq!(slot_0, expected_slot_0);

        // case 2- add tick that goes into the next slot
        tick_array.insert(100000);
        assert_eq!(tick_array.start_index_pending_update.unwrap(), 30);
        tick_array.flush();

        expected_bytes = vec![
            1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0,
            0, 160, 134, 1,
        ];
        assert_eq!(tick_array.cached_tick_bytes, expected_bytes);

        slot_0 = tick_array.slot_storage.sload(&key_0);
        let mut slot_1 = tick_array.slot_storage.sload(&key_1);

        expected_slot_0 = [
            1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0,
            0, 160, 134,
        ];
        let expected_slot_1: [u8; 32] = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert_eq!(slot_0, expected_slot_0);
        assert_eq!(slot_1, expected_slot_1);

        // case 3- removing tick clears slot
        tick_array.remove(1);

        assert_eq!(tick_array.start_index_pending_update.unwrap(), 0);
        tick_array.flush();

        expected_bytes = vec![
            2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0, 0, 160,
            134, 1, 0, 0, 0,
        ];
        assert_eq!(tick_array.cached_tick_bytes, expected_bytes);

        slot_0 = tick_array.slot_storage.sload(&key_0);
        slot_1 = tick_array.slot_storage.sload(&key_1);

        expected_slot_0 = [
            2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0, 0, 160,
            134, 1, 0, 0,
        ];
        let expected_slot_1: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert_eq!(slot_0, expected_slot_0);
        assert_eq!(slot_1, expected_slot_1);
    }

    #[test]
    fn ticks_decoded_correctly_from_slot() {
        let mut slot_storage = SlotStorage::new();
        let market_index = 0;

        let key_0 = TickArrayKey {
            market_index,
            tick_slot_index: 0,
        }
        .get_key();

        let slot_0: [u8; 32] = [
            1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0,
            0, 0, 0,
        ];

        slot_storage.sstore(&key_0, &slot_0);

        let mut tick_array = TickArray::new(market_index, slot_storage, true);
        tick_array.load_ticks();

        assert_eq!(tick_array.cached_tick_bytes, slot_0.to_vec());
        assert_eq!(tick_array.ticks(0), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
