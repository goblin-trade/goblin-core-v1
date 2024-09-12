use crate::state::{
    iterator::position::outer_index::{OuterIndexPosition, OuterIndexPositionIterator},
    ListKey, ListSlot, OuterIndex, Side, SlotStorage,
};

/// Read outer indices from the index list, end first.
/// In an index list, indices closer to the centre are at the end while
/// indices that are away are at the beginning of the list. That is
///
/// - bids are in ascending order
/// - asks are in descending order
///
pub struct ActiveOuterIndexIterator {
    /// Whether bid or ask. There are two lists, one for bids and one for asks.
    pub side: Side,

    inner: OuterIndexPositionIterator,

    // /// Number of indices yet to be read
    // pub outer_index_count: u16,
    /// The currently read list slot
    pub list_slot: Option<ListSlot>,
}

impl ActiveOuterIndexIterator {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            inner: OuterIndexPositionIterator { outer_index_count },
            list_slot: None, // Initialize with None
            side,
        }
    }

    pub fn outer_index_count(&self) -> u16 {
        self.inner.outer_index_count
    }

    /// Update the cached list slot if no value was cached or if we reached
    /// a new slot
    ///
    /// # Arguments
    ///
    /// * `outer_index_position`
    /// * `slot_storage`
    ///
    pub fn update_cached_list_slot(
        &mut self,
        outer_index_position: OuterIndexPosition,
        slot_storage: &SlotStorage,
    ) {
        let OuterIndexPosition {
            slot_index,
            relative_index,
        } = outer_index_position;

        if self.list_slot.is_none() || relative_index == 15 {
            let list_key = ListKey {
                index: slot_index,
                side: self.side,
            };
            self.list_slot = Some(ListSlot::new_from_slot(slot_storage, list_key));
        }
    }

    /// Read the next outer index
    ///
    /// # Arguments
    ///
    /// * slot_storage
    ///
    pub fn next(&mut self, slot_storage: &SlotStorage) -> Option<OuterIndex> {
        if let Some(outer_index_position) = self.inner.next() {
            self.update_cached_list_slot(outer_index_position, slot_storage);
            let list_slot = self.list_slot.as_ref().unwrap();
            let current_outer_index = list_slot.get(outer_index_position.relative_index as usize);

            return Some(current_outer_index);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::state::SlotActions;

    use super::*;

    #[test]
    fn test_empty_list() {
        let slot_storage = SlotStorage::new();
        let outer_index_count = 0;
        let side = Side::Bid;

        let mut reader = ActiveOuterIndexIterator::new(side, outer_index_count);
        assert!(reader.next(&slot_storage).is_none());
        assert!(reader.list_slot.is_none());
    }

    #[test]
    fn test_reader_single_slot() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let slot_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, slot_key);

        // Fill the list_slot with some values for testing
        list_slot.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot.write_to_slot(&mut slot_storage, &slot_key);

        // We are mocking the behavior, so just test that the reader works
        let outer_index_count = 16; // Only one slot needed
        let mut iterator = ActiveOuterIndexIterator::new(side, outer_index_count);

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
            // Obtain slot_index() and relative_index() of the upcoming value by calling
            // it before .next()
            assert_eq!(iterator.inner.slot_index(), expected.0);
            assert_eq!(iterator.inner.relative_index(), expected.1);

            let result = iterator.next(&slot_storage).unwrap();
            assert_eq!(result, expected.3);
            assert_eq!(iterator.list_slot, Some(expected.2));
        }

        assert!(iterator.next(&slot_storage).is_none());
    }

    #[test]
    fn test_reader_single_slot_partially_full() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let slot_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, slot_key);

        // Fill the list_slot with some values for testing
        list_slot.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, u16::MAX];
        list_slot.write_to_slot(&mut slot_storage, &slot_key);

        // We are mocking the behavior, so just test that the reader works
        let outer_index_count = 15; // Only one slot needed
        let mut iterator = ActiveOuterIndexIterator::new(side, outer_index_count);

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
            // Obtain slot_index() and relative_index() of the upcoming value by calling
            // it before .next()
            assert_eq!(iterator.inner.slot_index(), expected.0);
            assert_eq!(iterator.inner.relative_index(), expected.1);

            let result = iterator.next(&slot_storage).unwrap();
            assert_eq!(result, expected.3);
            assert_eq!(iterator.list_slot, Some(expected.2));
        }

        assert!(iterator.next(&slot_storage).is_none());
    }

    #[test]
    fn test_reader_multiple_slots() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let slot_key_0 = ListKey { index: 0, side };

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        list_slot_0.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot_0.write_to_slot(&mut slot_storage, &slot_key_0);

        let slot_key_1 = ListKey { index: 1, side };
        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, slot_key_1);
        list_slot_1.inner = [
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        list_slot_1.write_to_slot(&mut slot_storage, &slot_key_1);

        // Mock outer index count that spans across two slots
        let outer_index_count = 32;
        let mut iterator = ActiveOuterIndexIterator::new(side, outer_index_count);

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
            assert_eq!(iterator.inner.slot_index(), expected.0);
            assert_eq!(iterator.inner.relative_index(), expected.1);

            let result = iterator.next(&slot_storage).unwrap();
            assert_eq!(result, expected.3);
            assert_eq!(iterator.list_slot, Some(expected.2));
        }

        assert!(iterator.next(&slot_storage).is_none());
    }

    #[test]
    fn test_iterator_multiple_slots_partially_full() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;
        let slot_key_0 = ListKey { index: 0, side };
        let slot_key_1 = ListKey { index: 1, side };

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, slot_key_0);
        list_slot_0.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        list_slot_0.write_to_slot(&mut slot_storage, &slot_key_0);

        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, slot_key_1);
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
        list_slot_1.write_to_slot(&mut slot_storage, &slot_key_1);

        // Mock outer index count that spans across two slots
        let outer_index_count = 31;
        let mut iterator = ActiveOuterIndexIterator::new(side, outer_index_count);

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
            assert_eq!(iterator.inner.slot_index(), expected.0);
            assert_eq!(iterator.inner.relative_index(), expected.1);

            let result = iterator.next(&slot_storage).unwrap();
            assert_eq!(result, expected.3);
            assert_eq!(iterator.list_slot, Some(expected.2));
        }

        assert!(iterator.next(&slot_storage).is_none());
    }

    #[test]
    fn test_iterator_single_slot_descending_for_asks() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;
        let slot_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::new_from_slot(&slot_storage, slot_key);

        // Fill the list_slot with some values for testing
        list_slot.inner = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        list_slot.write_to_slot(&mut slot_storage, &slot_key);

        // Mocking the behavior, so just test that the iterator works
        let outer_index_count = 16; // Only one slot needed
        let mut iterator = ActiveOuterIndexIterator::new(side, outer_index_count);

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
            assert_eq!(iterator.inner.slot_index(), expected.0);
            assert_eq!(iterator.inner.relative_index(), expected.1);

            let result = iterator.next(&slot_storage).unwrap();
            assert_eq!(result, expected.3);
            assert_eq!(iterator.list_slot, Some(expected.2));
        }

        assert!(iterator.next(&slot_storage).is_none());
    }
}
