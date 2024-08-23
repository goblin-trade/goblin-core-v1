use crate::state::SlotStorage;

use super::{ListKey, ListSlot, OuterIndex};

pub struct IndexListIterator<'a> {
    pub slot_storage: &'a mut SlotStorage, // Reference to the slot storage
    pub outer_index_count: u16,            // Remaining elements to iterate
    pub list_slot: Option<ListSlot>,       // Cache the current list_slot
}

impl<'a> IndexListIterator<'a> {
    pub fn new(outer_index_count: u16, slot_storage: &'a mut SlotStorage) -> Self {
        Self {
            slot_storage,
            outer_index_count,
            list_slot: None, // Initialize with None
        }
    }
}

impl<'a> Iterator for IndexListIterator<'a> {
    type Item = (u16, u16, ListSlot, OuterIndex);

    fn next(&mut self) -> Option<Self::Item> {
        if self.outer_index_count == 0 {
            return None; // End iteration if no elements left
        }

        // Calculate slot index and relative index
        let slot_index = (self.outer_index_count - 1) / 16;
        let relative_index = (self.outer_index_count - 1) % 16;

        // Check if we need to load a new list_slot
        if self.list_slot.is_none() || relative_index == 15 {
            let list_key = ListKey { index: slot_index };
            self.list_slot = Some(ListSlot::new_from_slot(self.slot_storage, list_key));
        }

        // Safe to unwrap because we just initialized it if it was None
        let list_slot = self.list_slot.as_ref().unwrap();

        // Read the outer index from the list slot
        let current_outer_index = list_slot.get(relative_index as usize);

        // Prepare the result
        let result = (slot_index, relative_index, *list_slot, current_outer_index);

        // Decrement the outer_index_count for the next iteration
        self.outer_index_count -= 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::SlotActions;

    use super::*;

    #[test]
    fn test_empty_list() {
        let mut slot_storage = SlotStorage::new();
        let outer_index_count = 0;
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        assert!(iterator.next().is_none());
    }
    #[test]
    fn test_iterator_single_slot() {
        let mut slot_storage = SlotStorage::new();
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });

        // Fill the list_slot with some values for testing
        list_slot.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });

        // We are mocking the behavior, so just test that the iterator works
        let outer_index_count = 16; // Only one slot needed
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        let expected_results = vec![
            (0, 15, list_slot, OuterIndex::new(16)),
            (0, 14, list_slot, OuterIndex::new(15)),
            (0, 13, list_slot, OuterIndex::new(14)),
            (0, 12, list_slot, OuterIndex::new(13)),
            (0, 11, list_slot, OuterIndex::new(12)),
            (0, 10, list_slot, OuterIndex::new(11)),
            (0, 9, list_slot, OuterIndex::new(10)),
            (0, 8, list_slot, OuterIndex::new(9)),
            (0, 7, list_slot, OuterIndex::new(8)),
            (0, 6, list_slot, OuterIndex::new(7)),
            (0, 5, list_slot, OuterIndex::new(6)),
            (0, 4, list_slot, OuterIndex::new(5)),
            (0, 3, list_slot, OuterIndex::new(4)),
            (0, 2, list_slot, OuterIndex::new(3)),
            (0, 1, list_slot, OuterIndex::new(2)),
            (0, 0, list_slot, OuterIndex::new(1)),
        ];

        for expected in expected_results {
            let result = iterator.next().unwrap();
            assert_eq!(result, expected);
        }

        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_iterator_single_slot_partially_full() {
        let mut slot_storage = SlotStorage::new();
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });

        // Fill the list_slot with some values for testing
        list_slot.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, u16::MAX];
        list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });

        // We are mocking the behavior, so just test that the iterator works
        let outer_index_count = 15; // Only one slot needed
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        let expected_results = vec![
            (0, 14, list_slot, OuterIndex::new(15)),
            (0, 13, list_slot, OuterIndex::new(14)),
            (0, 12, list_slot, OuterIndex::new(13)),
            (0, 11, list_slot, OuterIndex::new(12)),
            (0, 10, list_slot, OuterIndex::new(11)),
            (0, 9, list_slot, OuterIndex::new(10)),
            (0, 8, list_slot, OuterIndex::new(9)),
            (0, 7, list_slot, OuterIndex::new(8)),
            (0, 6, list_slot, OuterIndex::new(7)),
            (0, 5, list_slot, OuterIndex::new(6)),
            (0, 4, list_slot, OuterIndex::new(5)),
            (0, 3, list_slot, OuterIndex::new(4)),
            (0, 2, list_slot, OuterIndex::new(3)),
            (0, 1, list_slot, OuterIndex::new(2)),
            (0, 0, list_slot, OuterIndex::new(1)),
        ];

        for expected in expected_results {
            let result = iterator.next().unwrap();
            assert_eq!(result, expected);
        }

        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_iterator_multiple_slots() {
        let mut slot_storage = SlotStorage::new();

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });
        list_slot_0.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot_0.write_to_slot(&mut slot_storage, &ListKey { index: 0 });

        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 1 });
        list_slot_1.inner = [
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        list_slot_1.write_to_slot(&mut slot_storage, &ListKey { index: 1 });

        // Mock outer index count that spans across two slots
        let outer_index_count = 32;
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        let expected_results = vec![
            (1, 15, list_slot_1, OuterIndex::new(32)),
            (1, 14, list_slot_1, OuterIndex::new(31)),
            (1, 13, list_slot_1, OuterIndex::new(30)),
            (1, 12, list_slot_1, OuterIndex::new(29)),
            (1, 11, list_slot_1, OuterIndex::new(28)),
            (1, 10, list_slot_1, OuterIndex::new(27)),
            (1, 9, list_slot_1, OuterIndex::new(26)),
            (1, 8, list_slot_1, OuterIndex::new(25)),
            (1, 7, list_slot_1, OuterIndex::new(24)),
            (1, 6, list_slot_1, OuterIndex::new(23)),
            (1, 5, list_slot_1, OuterIndex::new(22)),
            (1, 4, list_slot_1, OuterIndex::new(21)),
            (1, 3, list_slot_1, OuterIndex::new(20)),
            (1, 2, list_slot_1, OuterIndex::new(19)),
            (1, 1, list_slot_1, OuterIndex::new(18)),
            (1, 0, list_slot_1, OuterIndex::new(17)),
            (0, 15, list_slot_0, OuterIndex::new(16)),
            (0, 14, list_slot_0, OuterIndex::new(15)),
            (0, 13, list_slot_0, OuterIndex::new(14)),
            (0, 12, list_slot_0, OuterIndex::new(13)),
            (0, 11, list_slot_0, OuterIndex::new(12)),
            (0, 10, list_slot_0, OuterIndex::new(11)),
            (0, 9, list_slot_0, OuterIndex::new(10)),
            (0, 8, list_slot_0, OuterIndex::new(9)),
            (0, 7, list_slot_0, OuterIndex::new(8)),
            (0, 6, list_slot_0, OuterIndex::new(7)),
            (0, 5, list_slot_0, OuterIndex::new(6)),
            (0, 4, list_slot_0, OuterIndex::new(5)),
            (0, 3, list_slot_0, OuterIndex::new(4)),
            (0, 2, list_slot_0, OuterIndex::new(3)),
            (0, 1, list_slot_0, OuterIndex::new(2)),
            (0, 0, list_slot_0, OuterIndex::new(1)),
        ];

        for expected in expected_results {
            let result = iterator.next().unwrap();
            assert_eq!(result, expected);
        }

        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_iterator_multiple_slots_partially_full() {
        let mut slot_storage = SlotStorage::new();

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });
        list_slot_0.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot_0.write_to_slot(&mut slot_storage, &ListKey { index: 0 });

        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, ListKey { index: 1 });
        list_slot_1.inner = [
            17,
            18,
            19,
            20,
            21,
            22,
            23,
            24,
            25,
            26,
            27,
            28,
            29,
            30,
            31,
            u16::MAX,
        ];
        list_slot_1.write_to_slot(&mut slot_storage, &ListKey { index: 1 });

        // Mock outer index count that spans across two slots
        let outer_index_count = 31;
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        let expected_results = vec![
            (1, 14, list_slot_1, OuterIndex::new(31)),
            (1, 13, list_slot_1, OuterIndex::new(30)),
            (1, 12, list_slot_1, OuterIndex::new(29)),
            (1, 11, list_slot_1, OuterIndex::new(28)),
            (1, 10, list_slot_1, OuterIndex::new(27)),
            (1, 9, list_slot_1, OuterIndex::new(26)),
            (1, 8, list_slot_1, OuterIndex::new(25)),
            (1, 7, list_slot_1, OuterIndex::new(24)),
            (1, 6, list_slot_1, OuterIndex::new(23)),
            (1, 5, list_slot_1, OuterIndex::new(22)),
            (1, 4, list_slot_1, OuterIndex::new(21)),
            (1, 3, list_slot_1, OuterIndex::new(20)),
            (1, 2, list_slot_1, OuterIndex::new(19)),
            (1, 1, list_slot_1, OuterIndex::new(18)),
            (1, 0, list_slot_1, OuterIndex::new(17)),
            (0, 15, list_slot_0, OuterIndex::new(16)),
            (0, 14, list_slot_0, OuterIndex::new(15)),
            (0, 13, list_slot_0, OuterIndex::new(14)),
            (0, 12, list_slot_0, OuterIndex::new(13)),
            (0, 11, list_slot_0, OuterIndex::new(12)),
            (0, 10, list_slot_0, OuterIndex::new(11)),
            (0, 9, list_slot_0, OuterIndex::new(10)),
            (0, 8, list_slot_0, OuterIndex::new(9)),
            (0, 7, list_slot_0, OuterIndex::new(8)),
            (0, 6, list_slot_0, OuterIndex::new(7)),
            (0, 5, list_slot_0, OuterIndex::new(6)),
            (0, 4, list_slot_0, OuterIndex::new(5)),
            (0, 3, list_slot_0, OuterIndex::new(4)),
            (0, 2, list_slot_0, OuterIndex::new(3)),
            (0, 1, list_slot_0, OuterIndex::new(2)),
            (0, 0, list_slot_0, OuterIndex::new(1)),
        ];

        for expected in expected_results {
            let result = iterator.next().unwrap();
            assert_eq!(result, expected);
        }

        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_iterator_single_slot_descending_for_asks() {
        let mut slot_storage = SlotStorage::new();
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });

        // Fill the list_slot with some values for testing
        list_slot.inner = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });

        // We are mocking the behavior, so just test that the iterator works
        let outer_index_count = 16; // Only one slot needed
        let mut iterator = IndexListIterator::new(outer_index_count, &mut slot_storage);

        let expected_results = vec![
            (0, 15, list_slot, OuterIndex::new(1)),
            (0, 14, list_slot, OuterIndex::new(2)),
            (0, 13, list_slot, OuterIndex::new(3)),
            (0, 12, list_slot, OuterIndex::new(4)),
            (0, 11, list_slot, OuterIndex::new(5)),
            (0, 10, list_slot, OuterIndex::new(6)),
            (0, 9, list_slot, OuterIndex::new(7)),
            (0, 8, list_slot, OuterIndex::new(8)),
            (0, 7, list_slot, OuterIndex::new(9)),
            (0, 6, list_slot, OuterIndex::new(10)),
            (0, 5, list_slot, OuterIndex::new(11)),
            (0, 4, list_slot, OuterIndex::new(12)),
            (0, 3, list_slot, OuterIndex::new(13)),
            (0, 2, list_slot, OuterIndex::new(14)),
            (0, 1, list_slot, OuterIndex::new(15)),
            (0, 0, list_slot, OuterIndex::new(16)),
        ];

        for expected in expected_results {
            let result = iterator.next().unwrap();
            assert_eq!(result, expected);
        }

        assert!(iterator.next().is_none());
    }
}
