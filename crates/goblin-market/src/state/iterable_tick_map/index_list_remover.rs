use crate::state::{OuterIndex, Side, SlotStorage};
use alloc::vec::Vec;

use super::{write_prepared_indices, IndexListReader};

/// Enables bulk removal of outer indices from the index list.
/// Successive removed orders should be away from the center, i.e.,
///
/// - remove bids in ascending order
/// - remove asks in descending order
///
/// Removal can still leave ghost values in the index list. Use
/// MarketState::outer_index_count() to find the correct starting position.
/// Ghost values prevent slots from clearing thus saving gas.
///
/// Ghost values are produced when
///
/// 1. A slot was supposed to close. Instead values in the slot remain.
///
/// 2. Values are removed from the outermost slot but all values are not cleared.
/// Ghost values are because of the cached list slot. Values on right of the
/// removed values are copied into the space, but they are not cleared.
///
pub struct IndexListRemover {
    /// Iterator to read saved values from list
    pub index_list_reader: IndexListReader,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be retained after removal.
    pub cache: Vec<OuterIndex>,

    /// Tracks whether the searched outer index was found
    pub found_outer_index: Option<OuterIndex>,

    /// Whether one or more outer index was removed and the slots are pending a write.
    /// There are cases when we lookup outer indices with `find_outer_index()` but
    /// there are no updates to write.
    pub pending_write: bool,
}

impl IndexListRemover {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            index_list_reader: IndexListReader::new(side, outer_index_count),
            cache: Vec::new(),
            found_outer_index: None,
            pending_write: false,
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_reader.side
    }

    /// Traverse one position in the list
    /// The previous `found_outer_index` will be removed from list os it is discarded
    pub fn slide(&mut self, slot_storage: &SlotStorage) {
        self.found_outer_index.take();

        let gg = self.index_list_reader.next(slot_storage);
    }

    /// The total length of index list after accounting for removals
    pub fn index_list_length(&self) -> u16 {
        self.index_list_reader.outer_index_count
            + self.cache.len() as u16
            + u16::from(self.found_outer_index.is_some())
    }

    /// Clears `found_outer_index`, pushing the value to cache
    pub fn try_flush_found_outer_index_to_cache(&mut self) {
        if let Some(found_outer_index) = self.found_outer_index.take() {
            self.cache.push(found_outer_index);
        }
    }

    /// Searches for the outer index in the index list.
    ///
    /// # Arguments
    ///
    /// * `outer_index` - The index to search for
    /// * `slot_storage` - The slot storage to read indices from
    ///
    /// # Returns
    ///
    /// * `true` if the index is found, `false` otherwise.
    ///
    pub fn find_outer_index(
        &mut self,
        slot_storage: &SlotStorage,
        outer_index: OuterIndex,
    ) -> bool {
        if self
            .found_outer_index
            .is_some_and(|found_outer_index| found_outer_index == outer_index)
        {
            return true;
        }
        // Flush the old value of `found_outer_index` to cache
        self.try_flush_found_outer_index_to_cache();

        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_reader.next(slot_storage)
        {
            // Check if the current outer index matches the sought index
            if current_outer_index == outer_index {
                // Mark the outer index as found
                self.found_outer_index = Some(current_outer_index);
                return true;
            }
            // Cache indices that do not match
            self.cache.push(current_outer_index);
        }

        false
    }

    /// Prepare the index list by removing the specified outer index
    ///
    /// # Arguments
    ///
    /// * outer_index - The index to be removed
    /// * slot_storage - The slot storage to read indices from
    ///
    pub fn remove(&mut self, slot_storage: &SlotStorage, outer_index: OuterIndex) -> bool {
        // Find the element, then clear the found value
        if self.find_outer_index(slot_storage, outer_index) {
            self.pending_write = true;
            self.found_outer_index = None;
            return true;
        }
        false
    }

    /// Write prepared indices to slot after removal
    /// Externally ensure that `remove()` is called before writing to slot.
    /// Calling `write_prepared_indices()` after `find_outer_index()` will result
    /// in `found_outer_index`.
    pub fn write_prepared_indices(&mut self, slot_storage: &mut SlotStorage) {
        if !self.pending_write {
            return;
        }

        self.try_flush_found_outer_index_to_cache();
        write_prepared_indices(
            slot_storage,
            self.side(),
            &mut self.cache,
            self.index_list_reader.outer_index_count,
            self.index_list_reader.list_slot,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{ListKey, ListSlot, SlotActions};

    use super::*;

    #[test]
    fn test_find_outer_index_in_empty_list() {
        let slot_storage = SlotStorage::new();
        let mut remover = IndexListRemover::new(Side::Bid, 0);

        // Try to find an index in an empty list
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(!found);
        assert_eq!(remover.cache, vec![]);
    }

    #[test]
    fn test_find_existing_outer_index() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 1);

        // Find the existing element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(found);
        assert_eq!(remover.index_list_reader.outer_index_count, 0);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.found_outer_index, Some(OuterIndex::new(100)));
    }

    #[test]
    fn test_try_write_when_there_are_no_removals() {
        let mut slot_storage = SlotStorage::new();
        let list_key = ListKey {
            index: 0,
            side: Side::Bid,
        };
        // Setup the initial slot storage with one item
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let mut remover = IndexListRemover::new(Side::Bid, 1);

        // Find the existing element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(found);
        assert_eq!(remover.index_list_reader.outer_index_count, 0);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.found_outer_index, Some(OuterIndex::new(100)));
        assert_eq!(remover.pending_write, false);

        remover.write_prepared_indices(&mut slot_storage);

        let read_list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(read_list_slot, list_slot);
    }

    #[test]
    fn test_find_nonexistent_outer_index() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 1);

        // Try to find a nonexistent element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(!found);
        assert_eq!(remover.index_list_reader.outer_index_count, 0);
        assert_eq!(remover.cache, vec![OuterIndex::new(100)]);
        assert_eq!(remover.found_outer_index, None);
    }

    #[test]
    fn test_find_outer_index_and_cache_intermediary_values() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.set(1, OuterIndex::new(200));
            list_slot.set(2, OuterIndex::new(300));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 3);

        // Try to find the last element, cache intermediary values
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(found);
        assert_eq!(remover.index_list_reader.outer_index_count, 1);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.found_outer_index, Some(OuterIndex::new(200)));
    }

    #[test]
    fn test_remove_from_empty_list() {
        let slot_storage = SlotStorage::new();
        let mut remover = IndexListRemover::new(Side::Bid, 0);

        // Try to remove from an empty list
        let removed = remover.remove(&slot_storage, OuterIndex::new(100));
        assert_eq!(remover.index_list_length(), 0);
        assert!(!removed);
        assert_eq!(remover.cache, vec![]);
    }

    #[test]
    fn test_remove_nonexistent_element() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 1);

        // Attempt to remove an element that does not exist
        let removed = remover.remove(&slot_storage, OuterIndex::new(200));
        assert_eq!(remover.index_list_length(), 1);
        assert!(!removed);
        assert_eq!(remover.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_clear_slot_with_single_value() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 1);

        let removed = remover.remove(&slot_storage, OuterIndex::new(100));
        assert!(removed);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.index_list_reader.outer_index_count, 0);

        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot = ListSlot::new_from_slot(
            &slot_storage,
            ListKey {
                index: 0,
                side: Side::Bid,
            },
        );
        // Ghost values due to cleared slot
        assert_eq!(
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot.inner
        );
    }

    #[test]
    fn test_remove_element_from_single_slot_but_slot_is_not_cleared() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.set(1, OuterIndex::new(200));
            list_slot.set(2, OuterIndex::new(300));
            list_slot.write_to_slot(
                &mut slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut remover = IndexListRemover::new(Side::Bid, 3);

        // Remove the middle element (200)
        let removed = remover.remove(&slot_storage, OuterIndex::new(200));
        assert_eq!(remover.index_list_length(), 2);
        assert!(removed);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.index_list_reader.outer_index_count, 1);

        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot = ListSlot::new_from_slot(
            &slot_storage,
            ListKey {
                index: 0,
                side: Side::Bid,
            },
        );
        // Ghost values due to cached slot
        assert_eq!(
            vec![100, 300, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot.inner
        );
    }

    #[test]
    fn test_remove_element_from_two_contiguous_slots() {
        let mut slot_storage = SlotStorage::new();

        let list_key_0 = ListKey {
            index: 0,
            side: Side::Bid,
        };
        let list_key_1 = ListKey {
            index: 1,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot_0 = ListSlot::default();
            let mut list_slot_1 = ListSlot::default();
            list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            list_slot_1.inner = [16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
            list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 18);

        let removed = remover.remove(&slot_storage, OuterIndex::new(15));
        assert!(removed);
        assert_eq!(
            remover.cache,
            vec![OuterIndex::new(17), OuterIndex::new(16)]
        );
        assert_eq!(remover.index_list_reader.outer_index_count, 15);
        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);

        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16],
            result_slot_0.inner
        );
        // No ghost value because the slot was not cached
        assert_eq!(
            vec![17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot_1.inner
        );
    }

    #[test]
    fn test_remove_element_from_two_contiguous_slots_with_last_slot_cleared() {
        let mut slot_storage = SlotStorage::new();

        let list_key_0 = ListKey {
            index: 0,
            side: Side::Bid,
        };
        let list_key_1 = ListKey {
            index: 1,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot_0 = ListSlot::default();
            let mut list_slot_1 = ListSlot::default();
            list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            list_slot_1.inner = [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
            list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 17);

        let removed = remover.remove(&slot_storage, OuterIndex::new(15));
        assert!(removed);
        assert_eq!(remover.cache, vec![OuterIndex::new(16)]);
        assert_eq!(remover.index_list_reader.outer_index_count, 15);
        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);

        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16],
            result_slot_0.inner
        );
        // Ghost value because slot was cleared
        assert_eq!(
            vec![16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot_1.inner
        );
    }

    #[test]
    fn test_remove_elements_from_two_contiguous_slots() {
        let mut slot_storage = SlotStorage::new();

        let list_key_0 = ListKey {
            index: 0,
            side: Side::Bid,
        };
        let list_key_1 = ListKey {
            index: 1,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot_0 = ListSlot::default();
            let mut list_slot_1 = ListSlot::default();
            list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            list_slot_1.inner = [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
            list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 17);

        let removed_15 = remover.remove(&slot_storage, OuterIndex::new(15));
        assert!(removed_15);
        assert_eq!(remover.cache, vec![OuterIndex::new(16)]);
        assert_eq!(remover.index_list_reader.outer_index_count, 15);

        let removed_12 = remover.remove(&slot_storage, OuterIndex::new(12));
        assert!(removed_12);
        assert_eq!(
            remover.cache,
            vec![
                OuterIndex::new(16),
                OuterIndex::new(14),
                OuterIndex::new(13)
            ]
        );
        assert_eq!(remover.index_list_reader.outer_index_count, 12);

        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);

        // Ghost value due to cached slot
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16, 15],
            result_slot_0.inner
        );
        // Ghost value due to cleared slot
        assert_eq!(
            vec![16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot_1.inner
        );
    }

    #[test]
    fn test_remove_elements_from_non_contiguous_slots() {
        let mut slot_storage = SlotStorage::new();

        let list_key_0 = ListKey {
            index: 0,
            side: Side::Bid,
        };
        let list_key_1 = ListKey {
            index: 1,
            side: Side::Bid,
        };
        let list_key_2 = ListKey {
            index: 2,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot_0 = ListSlot::default();
            let mut list_slot_1 = ListSlot::default();
            let mut list_slot_2 = ListSlot::default();
            list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            list_slot_1.inner = [
                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ];
            list_slot_2.inner = [32, 33, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
            list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);
            list_slot_2.write_to_slot(&mut slot_storage, &list_key_2);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 35);

        let removed_33 = remover.remove(&slot_storage, OuterIndex::new(33));
        assert!(removed_33);
        assert_eq!(remover.cache, vec![OuterIndex::new(34)]);
        assert_eq!(remover.index_list_reader.outer_index_count, 33);

        let removed_12 = remover.remove(&slot_storage, OuterIndex::new(12));
        assert!(removed_12);
        assert_eq!(
            remover.cache,
            vec![
                34, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13
            ]
            .into_iter()
            .map(OuterIndex::new)
            .collect::<Vec<OuterIndex>>()
        );
        assert_eq!(remover.index_list_reader.outer_index_count, 12);

        remover.write_prepared_indices(&mut slot_storage);

        // Validate the contents of the first slot after removal
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        let result_slot_2 = ListSlot::new_from_slot(&slot_storage, list_key_2);

        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16],
            result_slot_0.inner
        );
        assert_eq!(
            vec![17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32],
            result_slot_1.inner
        );
        // Ghost value due to cleared slot
        assert_eq!(
            vec![34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            result_slot_2.inner
        );
    }

    #[test]
    fn test_find_and_remove_same_value() {
        let mut slot_storage = SlotStorage::new();
        let list_key = ListKey {
            index: 0,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.set(1, OuterIndex::new(200));
            list_slot.set(2, OuterIndex::new(300));
            list_slot.write_to_slot(&mut slot_storage, &list_key);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 3);

        // Find and remove the element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(found);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.found_outer_index, Some(OuterIndex::new(200)));

        remover.remove(&mut slot_storage, OuterIndex::new(200));
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert!(remover.found_outer_index.is_none());

        // Verify the state after write
        remover.write_prepared_indices(&mut slot_storage);
        let list_slot = ListSlot::new_from_slot(&slot_storage, list_key);

        assert_eq!(list_slot.get(0), OuterIndex::new(100));
        assert_eq!(list_slot.get(1), OuterIndex::new(300));
        assert_eq!(list_slot.get(2), OuterIndex::new(300)); // Ghost values due to cached slot
    }

    #[test]
    fn test_find_one_value_remove_another() {
        let mut slot_storage = SlotStorage::new();
        let list_key = ListKey {
            index: 0,
            side: Side::Bid,
        };

        // Setup the initial slot storage with multiple items
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.set(1, OuterIndex::new(200));
            list_slot.set(2, OuterIndex::new(300));
            list_slot.write_to_slot(&mut slot_storage, &list_key);
        }

        let mut remover = IndexListRemover::new(Side::Bid, 3);

        // Find the value but remove a different one
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(found);
        assert_eq!(remover.index_list_reader.outer_index_count, 1);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.found_outer_index, Some(OuterIndex::new(200)));

        remover.remove(&mut slot_storage, OuterIndex::new(100));
        assert_eq!(remover.index_list_reader.outer_index_count, 0);
        assert_eq!(
            remover.cache,
            vec![OuterIndex::new(300), OuterIndex::new(200)]
        );
        assert!(remover.found_outer_index.is_none());

        // Verify the state after write
        remover.write_prepared_indices(&mut slot_storage);
        let list_slot = ListSlot::new_from_slot(&slot_storage, list_key);

        assert_eq!(list_slot.get(0), OuterIndex::new(200));
        assert_eq!(list_slot.get(1), OuterIndex::new(300));
        assert_eq!(list_slot.get(2), OuterIndex::new(300)); // Ghost values due to cached slot
    }
}
