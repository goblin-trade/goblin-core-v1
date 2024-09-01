use super::{IndexListReader, ListKey, ListSlot};
use crate::state::{OuterIndex, Side, SlotStorage};
use alloc::vec::Vec;

/// Enables bulk insertion of outer indices in the index list.
/// Successive inserted orders should move away from the centre, i.e.
///
/// - insert bids in descending order
/// - insert asks in ascending order
///
pub struct IndexListInserter {
    /// Iterator to read saved values from list
    pub index_list_reader: IndexListReader,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be inserted and values popped from index list reader
    /// in the correct order of insertion.
    pub cache: Vec<OuterIndex>,
}

impl IndexListInserter {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            index_list_reader: IndexListReader::new(outer_index_count, side),
            cache: Vec::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_reader.side
    }

    /// Prepare an outer index for insertion in the index list
    ///
    /// # Arguments
    ///
    /// * outer_index
    /// * slot_storage
    ///
    pub fn prepare(&mut self, slot_storage: &SlotStorage, outer_index: OuterIndex) -> bool {
        // Check last element in the cache
        if let Some(&last_pushed_outer_index) = self.cache.last() {
            // If the element already exists in the cache, return false
            if last_pushed_outer_index == outer_index {
                return false;
            }

            // If the last element in cache is closer to the center than outer_index, insert before it
            if last_pushed_outer_index.is_closer_to_center(self.side(), outer_index) {
                self.cache.pop(); // Remove the last pushed index
                self.cache.push(outer_index);
                self.cache.push(last_pushed_outer_index); // Push it back after the new index
                return true;
            }
        }

        // Iterate through the list to find the correct position
        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_reader.next(slot_storage)
        {
            // If the outer_index is already in the list, only insert once
            if current_outer_index == outer_index {
                self.cache.push(current_outer_index);
                return false;
            }

            // If outer_index is closer to the center, insert before current_outer_index
            if current_outer_index.is_closer_to_center(self.side(), outer_index) {
                self.cache.push(outer_index);
                self.cache.push(current_outer_index);
                return true;
            } else {
                // Otherwise, push the current_outer_index to cache and continue
                self.cache.push(current_outer_index);
            }
        }

        // If reached end without finding a suitable position, push the outer_index to cache
        self.cache.push(outer_index);
        true
    }

    /// Write prepared indices to slot
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

/// Write cached indices to slot
///
/// This must be called after calling prepare() atlreast once, else the function fails
/// because index_list_iterator.list_slot.unwrap() will throw error.
///
/// Indices are written from the left (start) to right.
///
pub fn write_prepared_indices(
    slot_storage: &mut SlotStorage,
    side: Side,
    cache: &mut Vec<OuterIndex>,
    unread_count: u16,
    first_list_slot: Option<ListSlot>,
) {
    if cache.is_empty() {
        return;
    }

    let start_slot_index = unread_count / 16;

    let size_after_insertions = unread_count + cache.len() as u16;
    let final_slot_index_inclusive = (size_after_insertions - 1) / 16;

    for slot_index in start_slot_index..=final_slot_index_inclusive {
        let (mut list_slot, start_relative_index) = if slot_index == start_slot_index {
            (first_list_slot.unwrap_or_default(), unread_count % 16)
        } else {
            (ListSlot::default(), 0)
        };

        let final_relative_index_inclusive = if slot_index == final_slot_index_inclusive {
            (size_after_insertions - 1) % 16
        } else {
            15
        };

        for relative_index in start_relative_index..=final_relative_index_inclusive {
            let outer_index = cache.pop().unwrap();
            list_slot.set(relative_index as usize, outer_index);
        }

        list_slot.write_to_slot(
            slot_storage,
            &ListKey {
                index: slot_index,
                side,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::state::SlotActions;

    use super::*;

    #[test]
    fn test_prepare_bid_empty_list() {
        let slot_storage = &mut SlotStorage::new();
        let mut insertion = IndexListInserter::new(Side::Bid, 0);

        // Insert into an empty list
        assert!(insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        // Externally ensure that subsequent indices move away from the centre.
        // This case is to deal with the last value from .next()
        assert!(insertion.prepare(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![
                OuterIndex::new(150),
                OuterIndex::new(100),
                OuterIndex::new(50)
            ]
        );
    }

    #[test]
    fn test_prepare_bid_equal_index() {
        let mut slot_storage = &mut SlotStorage::new();

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

        let mut insertion = IndexListInserter::new(Side::Bid, 1);

        // Attempt to insert the same index
        assert!(!insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_bid_closer_to_center() {
        let slot_storage = &mut SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(
                slot_storage,
                &ListKey {
                    index: 0,
                    side: Side::Bid,
                },
            );
        }

        let mut insertion = IndexListInserter::new(Side::Bid, 1);

        // Insert an index closer to the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_bid_away_from_center() {
        let mut slot_storage = &mut SlotStorage::new();

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

        let mut insertion = IndexListInserter::new(Side::Bid, 1);

        // Insert an index further away from the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(50)]
        );
    }

    #[test]
    fn test_prepare_ask_empty_list() {
        let slot_storage = &mut SlotStorage::new();
        let mut insertion = IndexListInserter::new(Side::Ask, 0);

        // Insert into an empty list
        assert!(insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![
                OuterIndex::new(50),
                OuterIndex::new(100),
                OuterIndex::new(150)
            ]
        );
    }

    #[test]
    fn test_prepare_ask_equal_index() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = IndexListInserter::new(side, 1);

        // Attempt to insert the same index
        assert!(!insertion.prepare(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_ask_closer_to_center() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = IndexListInserter::new(side, 1);

        // Insert an index closer to the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_ask_away_from_center() {
        let slot_storage = &mut SlotStorage::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = IndexListInserter::new(side, 1);

        // Insert an index further away from the center
        assert!(insertion.prepare(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(150)]
        );
    }

    // Test commit function

    #[test]
    fn test_write_prepared_indices_basic_reverse_order() {
        let mut slot_storage = SlotStorage::new();
        let mut cache = vec![OuterIndex::new(1), OuterIndex::new(2), OuterIndex::new(3)];
        let unread_count = 0;
        let first_list_slot = None;
        let side = Side::Ask;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            first_list_slot,
        );

        // Validate the contents of the first slot
        let result_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0, side });
        assert_eq!(result_slot.get(0), OuterIndex::new(3));
        assert_eq!(result_slot.get(1), OuterIndex::new(2));
        assert_eq!(result_slot.get(2), OuterIndex::new(1));
        for i in 3..16 {
            assert_eq!(result_slot.get(i), OuterIndex::new(0)); // Default empty value
        }
    }

    #[test]
    fn test_write_prepared_indices_with_unread_count() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let mut first_list_slot = ListSlot::default();
        first_list_slot.set(0, OuterIndex::new(100)); // Existing unread index
        first_list_slot.write_to_slot(
            &mut slot_storage,
            &ListKey {
                index: 0,
                side: Side::Bid,
            },
        );

        let mut cache = vec![OuterIndex::new(300), OuterIndex::new(200)];
        let unread_count = 1;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            Some(first_list_slot),
        );

        // Validate the contents of the first slot
        let result_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0, side });
        assert_eq!(result_slot.get(0), OuterIndex::new(100)); // Unread index
        assert_eq!(result_slot.get(1), OuterIndex::new(200));
        assert_eq!(result_slot.get(2), OuterIndex::new(300));
        for i in 3..16 {
            assert_eq!(result_slot.get(i), OuterIndex::new(0)); // Default empty value
        }
    }

    #[test]
    fn test_write_prepared_indices_multi_slot() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let mut cache = vec![
            OuterIndex::new(1),
            OuterIndex::new(2),
            OuterIndex::new(3),
            OuterIndex::new(4),
            OuterIndex::new(5),
            OuterIndex::new(6),
            OuterIndex::new(7),
            OuterIndex::new(8),
            OuterIndex::new(9),
            OuterIndex::new(10),
            OuterIndex::new(11),
            OuterIndex::new(12),
            OuterIndex::new(13),
            OuterIndex::new(14),
            OuterIndex::new(15),
            OuterIndex::new(16),
            OuterIndex::new(17),
            OuterIndex::new(18),
            OuterIndex::new(19),
        ];
        let unread_count = 0;
        let first_list_slot = None;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            first_list_slot,
        );

        // Validate the contents of the first slot
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0, side });
        for i in 0..16 {
            assert_eq!(result_slot_0.get(i), OuterIndex::new(19 - i as u16));
        }

        // Validate the contents of the second slot
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 1, side });
        assert_eq!(result_slot_1.get(0), OuterIndex::new(3));
        assert_eq!(result_slot_1.get(1), OuterIndex::new(2));
        assert_eq!(result_slot_1.get(2), OuterIndex::new(1));
        for i in 3..16 {
            assert_eq!(result_slot_1.get(i), OuterIndex::new(0)); // Default empty value
        }
    }

    #[test]
    fn test_write_prepared_indices_multi_slot_with_slot_0_partially_full() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        let slot_key_0 = ListKey { index: 0, side };
        let slot_key_1 = ListKey { index: 1, side };

        // Prepopulate the first slot with some existing values (partially full)
        let mut first_list_slot = ListSlot::default();

        for i in 0..8 {
            first_list_slot.set(i, OuterIndex::new(100 + i as u16));
        }
        first_list_slot.write_to_slot(&mut slot_storage, &slot_key_0);

        let mut cache = vec![
            OuterIndex::new(1),
            OuterIndex::new(2),
            OuterIndex::new(3),
            OuterIndex::new(4),
            OuterIndex::new(5),
            OuterIndex::new(6),
            OuterIndex::new(7),
            OuterIndex::new(8),
            OuterIndex::new(9),
            OuterIndex::new(10),
            OuterIndex::new(11),
            OuterIndex::new(12),
        ];
        let unread_count = 8; // 8 unread items are already in slot 0

        // Write the cache to the slot storage
        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            Some(first_list_slot),
        );

        // Validate the contents of the first slot
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        // First 8 elements should be untouched
        for i in 0..8 {
            assert_eq!(result_slot_0.get(i), OuterIndex::new(100 + i as u16));
        }
        // Remaining 8 elements should be from the cache in reverse order
        for i in 8..16 {
            assert_eq!(result_slot_0.get(i), OuterIndex::new(20 - i as u16));
        }

        // Validate there are no additional slots written
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, slot_key_1);
        for i in 0..4 {
            assert_eq!(result_slot_1.get(i), OuterIndex::new(4 - i as u16));
        }

        for i in 4..16 {
            assert_eq!(result_slot_1.get(i), OuterIndex::new(0)); // Default empty value
        }
    }

    #[test]
    fn test_write_prepared_indices_multi_slot_with_slot_0_completely_full() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;
        let slot_key_0 = ListKey { index: 0, side };
        let slot_key_1 = ListKey { index: 1, side };
        let slot_key_2 = ListKey { index: 2, side };

        // Prepopulate the first slot with some existing values (completely full)
        let mut first_list_slot = ListSlot::default();
        for i in 0..16 {
            first_list_slot.set(i, OuterIndex::new(100 + i as u16));
        }
        first_list_slot.write_to_slot(&mut slot_storage, &slot_key_0);

        let mut cache = vec![
            OuterIndex::new(1),
            OuterIndex::new(2),
            OuterIndex::new(3),
            OuterIndex::new(4),
            OuterIndex::new(5),
            OuterIndex::new(6),
            OuterIndex::new(7),
            OuterIndex::new(8),
            OuterIndex::new(9),
            OuterIndex::new(10),
            OuterIndex::new(11),
            OuterIndex::new(12),
            OuterIndex::new(13),
            OuterIndex::new(14),
            OuterIndex::new(15),
            OuterIndex::new(16),
            OuterIndex::new(17),
            OuterIndex::new(18),
            OuterIndex::new(19),
            OuterIndex::new(20),
            OuterIndex::new(21),
            OuterIndex::new(22),
            OuterIndex::new(23),
            OuterIndex::new(24),
            OuterIndex::new(25),
            OuterIndex::new(26),
            OuterIndex::new(27),
            OuterIndex::new(28),
            OuterIndex::new(29),
            OuterIndex::new(30),
            OuterIndex::new(31),
            OuterIndex::new(32),
        ];

        let unread_count = 16;

        // Write the cache to the slot storage
        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            Some(first_list_slot),
        );

        // Validate the contents of the first slot
        let result_slot_0 = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        for i in 0..16 {
            assert_eq!(result_slot_0.get(i), OuterIndex::new(100 + i as u16)); // Descending order
        }

        // Validate the contents of the second slot
        let result_slot_1 = ListSlot::new_from_slot(&slot_storage, slot_key_1);
        for i in 0..16 {
            assert_eq!(result_slot_1.get(i), OuterIndex::new(32 - i as u16));
        }

        // Validate the contents of the third slot
        let result_slot_2 = ListSlot::new_from_slot(&slot_storage, slot_key_2);
        for i in 0..16 {
            assert_eq!(result_slot_2.get(i), OuterIndex::new(16 - i as u16));
        }
    }

    #[test]
    fn test_write_prepared_indices_edge_case_exact_slot() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;
        let slot_key_0 = ListKey { index: 0, side };

        let mut cache = vec![
            OuterIndex::new(1),
            OuterIndex::new(2),
            OuterIndex::new(3),
            OuterIndex::new(4),
            OuterIndex::new(5),
            OuterIndex::new(6),
            OuterIndex::new(7),
            OuterIndex::new(8),
            OuterIndex::new(9),
            OuterIndex::new(10),
            OuterIndex::new(11),
            OuterIndex::new(12),
            OuterIndex::new(13),
            OuterIndex::new(14),
            OuterIndex::new(15),
            OuterIndex::new(16),
        ];
        let unread_count = 0;
        let first_list_slot = None;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            first_list_slot,
        );

        // Validate the contents of the first slot
        let result_slot = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        for i in 0..16 {
            assert_eq!(result_slot.get(i), OuterIndex::new(16 - i as u16));
        }
    }

    #[test]
    fn test_write_prepared_indices_empty_cache() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;
        let slot_key_0 = ListKey { index: 0, side };

        let mut first_list_slot = ListSlot::default();
        first_list_slot.set(0, OuterIndex::new(100)); // Existing unread index
        first_list_slot.write_to_slot(&mut slot_storage, &slot_key_0);

        let mut cache = vec![];
        let unread_count = 1;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            Some(first_list_slot),
        );

        // Validate that nothing has changed
        let result_slot = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        assert_eq!(result_slot.get(0), OuterIndex::new(100)); // Unread index
        for i in 1..16 {
            assert_eq!(result_slot.get(i), OuterIndex::new(0)); // Default empty value
        }
    }

    #[test]
    fn test_write_prepared_indices_partial_slot_with_unread() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let slot_key_0 = ListKey { index: 0, side };

        let mut first_list_slot = ListSlot::default();
        first_list_slot.set(0, OuterIndex::new(100)); // Existing unread index
        first_list_slot.set(1, OuterIndex::new(200)); // Another unread index
        first_list_slot.write_to_slot(&mut slot_storage, &slot_key_0);

        let mut cache = vec![OuterIndex::new(300), OuterIndex::new(400)];
        let unread_count = 2;

        write_prepared_indices(
            &mut slot_storage,
            side,
            &mut cache,
            unread_count,
            Some(first_list_slot),
        );

        // Validate the contents of the first slot
        let result_slot = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        assert_eq!(result_slot.get(0), OuterIndex::new(100)); // Unread index
        assert_eq!(result_slot.get(1), OuterIndex::new(200)); // Unread index
        assert_eq!(result_slot.get(2), OuterIndex::new(400));
        assert_eq!(result_slot.get(3), OuterIndex::new(300));
        for i in 4..16 {
            assert_eq!(result_slot.get(i), OuterIndex::new(0)); // Default empty value
        }
    }
}
