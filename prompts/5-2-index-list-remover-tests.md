rs
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
}

impl IndexListRemover {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            index_list_reader: IndexListReader::new(outer_index_count, side),
            cache: Vec::new(),
            found_outer_index: None,
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_reader.side
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
        if let Some(found_outer_index) = self.found_outer_index {
            if found_outer_index == outer_index {
                return true;
            }

            self.cache.push(found_outer_index);
            self.found_outer_index = None;
        }

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
            self.found_outer_index = None;
            return true;
        }
        false
    }

    /// Write prepared indices to slot after removal
    pub fn write_prepared_indices(&mut self, slot_storage: &mut SlotStorage) {
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
    fn test_remove_from_empty_list() {
        let slot_storage = SlotStorage::new();
        let mut remover = IndexListRemover::new(Side::Bid, 0);

        // Try to remove from an empty list
        let removed = remover.remove(&slot_storage, OuterIndex::new(100));
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
        println!("result slot {:?}", result_slot);
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
}


The remove function was updated to use an intermediary function find_outer_index().

- Write tests for find_outer_index()
- Potentially update some of the tests for remove()

Definitions
- A slot is made of a 256 bit key and 256 bit value
- A list slot consists of 16 outer indices with an inner u16.
- List slots make up an index list. Outer indices in the list are sorted.
