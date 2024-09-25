use crate::state::{
    iterator::active_position::active_outer_index_iterator::ActiveOuterIndexIterator,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};
use alloc::vec::Vec;

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
/// 1. A slot was supposed to close. Instead all values in the slot become ghost
/// values. We can simply decrement outer index count in the market state and
/// ignore these values.
///
/// 2. Special case of (1) where there is no slot write. The last value of the outermost was removed.
/// There is no need to perform any slot write, simply decrement the outer index count.
///
/// 3. When the 'currently cached slot' does not close. Values that were supposed to
/// get be shifted left instead get duplicated to the left without leaving their
/// original place. Note- this can combine with (1) when we the inner slot does not
/// close but the outer slot does.
///
pub struct OuterIndexRemover {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIterator,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be retained after removal.
    pub cache: Vec<OuterIndex>,

    /// Staging area to efficiently lookup and remove outer indices. It is set with `slide()`
    /// where we read the outermost outer index. There are two possibilities for this staged value
    /// 1. `cached_outer_index` was the value being searched for removal. Remove it.
    /// 2. Slide futher. This will update `cached_outer_index`
    pub cached_outer_index: Option<OuterIndex>,

    /// Whether one or more outer index was removed and the slots are pending a write.
    /// There are cases when we lookup outer indices with `find_outer_index()` but
    /// there are no updates to write.
    pub pending_write: bool,
}

impl OuterIndexRemover {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIterator::new(side, outer_index_count),
            cache: Vec::new(),
            cached_outer_index: None,
            pending_write: false,
        }
    }

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }

    /// The total length of index list after accounting for removals
    pub fn index_list_length(&self) -> u16 {
        self.active_outer_index_iterator.outer_index_count()
            + self.cache.len() as u16
            + u16::from(self.cached_outer_index.is_some())
    }

    /// Traverse one position down the list pushing the previous `cached_outer_index` to cache
    /// if it exists and writing the read outer index to `cached_outer_index`.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    ///
    pub fn slide(&mut self, slot_storage: &ArbContext) {
        self.flush_cached_outer_index();
        self.cached_outer_index = self.active_outer_index_iterator.next(slot_storage);
    }

    /// Pushes `found_outer_index` to cache and clears the value
    pub fn flush_cached_outer_index(&mut self) {
        if let Some(found_outer_index) = self.cached_outer_index.take() {
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
    pub fn find_outer_index(&mut self, slot_storage: &ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if self.cached_outer_index == Some(outer_index) {
                return true;
            }
            self.slide(slot_storage);
            if self.cached_outer_index.is_none() {
                return false;
            }
        }
    }

    /// Remove the cached index, and set `pending_write` to true if the cached list
    /// is not empty
    ///
    pub fn remove_cached_index(&mut self) {
        self.cached_outer_index = None;
        self.pending_write = !self.cache.is_empty();
    }

    /// Write prepared indices to slot after removal
    ///
    /// Externally ensure that `remove()` is called before writing to slot.
    /// Calling `write_prepared_indices()` after `find_outer_index()` will result
    /// in `found_outer_index`.
    ///
    pub fn write_index_list(&mut self, slot_storage: &mut ArbContext) {
        if !self.pending_write {
            return;
        }

        self.flush_cached_outer_index();
        write_index_list(
            slot_storage,
            self.side(),
            &mut self.cache,
            self.active_outer_index_iterator.outer_index_count(),
            self.active_outer_index_iterator.list_slot,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{ContextActions, ListKey, ListSlot};

    use super::*;

    #[test]
    fn test_find_outer_index_in_empty_list() {
        let slot_storage = ArbContext::new();
        let mut remover = OuterIndexRemover::new(Side::Bid, 0);

        // Try to find an index in an empty list
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(!found);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.cached_outer_index, None);
        assert!(!remover.pending_write);
    }

    #[test]
    fn test_find_and_remove_outer_index() {
        let mut slot_storage = ArbContext::new();

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

        let mut remover = OuterIndexRemover::new(Side::Bid, 1);

        // Find the existing element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(found);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.cached_outer_index, Some(OuterIndex::new(100)));
        assert!(!remover.pending_write);

        remover.remove_cached_index();
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.cached_outer_index, None);
        assert!(!remover.pending_write); // false because cache is empty
    }

    #[test]
    fn test_find_one_but_remove_another() {
        let mut slot_storage = ArbContext::new();

        let side = Side::Bid;
        let outer_index_count = 2;

        // Setup the initial slot storage with one item
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.set(1, OuterIndex::new(200));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        // Find the existing element

        let found_200 = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(found_200);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 1);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.cached_outer_index, Some(OuterIndex::new(200)));
        assert!(!remover.pending_write);

        let found_100 = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(found_100);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        // Previous `cached_outer_index` is pushed to cache array
        assert_eq!(remover.cache, vec![OuterIndex::new(200)]);
        assert_eq!(remover.cached_outer_index, Some(OuterIndex::new(100)));
        assert!(!remover.pending_write);

        remover.remove_cached_index();
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        assert_eq!(remover.cache, vec![OuterIndex::new(200)]);
        assert_eq!(remover.cached_outer_index, None);
        assert!(remover.pending_write);
    }

    #[test]
    fn test_try_write_when_there_are_no_removals() {
        let mut slot_storage = ArbContext::new();
        let list_key = ListKey {
            index: 0,
            side: Side::Bid,
        };
        // Setup the initial slot storage with one item
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let mut remover = OuterIndexRemover::new(Side::Bid, 1);

        // Find the existing element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        assert!(found);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.cached_outer_index, Some(OuterIndex::new(100)));
        assert_eq!(remover.pending_write, false);

        remover.write_index_list(&mut slot_storage);

        let read_list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(read_list_slot, list_slot);
    }

    #[test]
    fn test_find_nonexistent_outer_index() {
        let mut slot_storage = ArbContext::new();

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

        let mut remover = OuterIndexRemover::new(Side::Bid, 1);

        // Try to find a nonexistent element
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(!found);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
        assert_eq!(remover.cache, vec![OuterIndex::new(100)]);
        assert_eq!(remover.cached_outer_index, None);
        assert!(!remover.pending_write);

        // Ensure this is not called if no `cached_outer_index` is present.
        // Else `pending_write` will be set to true.
        remover.remove_cached_index();
        assert!(remover.pending_write);
    }

    #[test]
    fn test_find_outer_index_and_cache_intermediary_values() {
        let mut slot_storage = ArbContext::new();

        // Setup the initial slot storage with multiple items
        let list_key = ListKey {
            index: 0,
            side: Side::Bid,
        };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.set(1, OuterIndex::new(200));
        list_slot.set(2, OuterIndex::new(300));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let mut remover = OuterIndexRemover::new(Side::Bid, 3);

        // Try to find the last element, cache intermediary values
        let found = remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        assert!(found);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 1);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.cached_outer_index, Some(OuterIndex::new(200)));
        assert!(!remover.pending_write);

        remover.remove_cached_index();
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 1);
        assert_eq!(remover.cache, vec![OuterIndex::new(300)]);
        assert_eq!(remover.cached_outer_index, None);
        assert!(remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot = ListSlot::new_from_slot(&slot_storage, list_key);

        let mut expected_list_slot = ListSlot::default();
        expected_list_slot.set(0, OuterIndex::new(100));
        expected_list_slot.set(1, OuterIndex::new(300));

        // Ghost value from cached slot
        expected_list_slot.set(2, OuterIndex::new(300));
        assert_eq!(read_list_slot, expected_list_slot);
    }

    #[test]
    fn test_remove_multiple_adjacent_outermost_in_same_slot() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 4;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };

        let mut list_slot_0 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);

        remover.find_outer_index(&slot_storage, OuterIndex::new(3));
        remover.remove_cached_index();

        remover.find_outer_index(&slot_storage, OuterIndex::new(2));
        remover.remove_cached_index();

        assert!(!remover.pending_write);
        assert_eq!(remover.cached_outer_index, None);
        assert_eq!(remover.cache, vec![]);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 2);
    }

    #[test]
    fn test_remove_multiple_adjacent_non_outermost_in_same_slot() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 4;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };

        let mut list_slot_0 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);

        remover.find_outer_index(&slot_storage, OuterIndex::new(2));
        remover.remove_cached_index();

        remover.find_outer_index(&slot_storage, OuterIndex::new(1));
        remover.remove_cached_index();

        assert!(remover.pending_write);
        assert_eq!(remover.cached_outer_index, None);
        assert_eq!(remover.cache, vec![OuterIndex::new(3)]);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 1);
    }

    #[test]
    fn test_remove_multiple_non_adjacent_in_same_slot() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 4;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };

        let mut list_slot_0 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);

        remover.find_outer_index(&slot_storage, OuterIndex::new(2));
        remover.remove_cached_index();

        remover.find_outer_index(&slot_storage, OuterIndex::new(0));
        remover.remove_cached_index();

        assert!(remover.pending_write);
        assert_eq!(remover.cached_outer_index, None);
        assert_eq!(remover.cache, vec![OuterIndex::new(3), OuterIndex::new(1)]);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 0);
    }

    #[test]
    fn test_remove_multiple_different_adjacent_slots() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 18;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };
        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        remover.find_outer_index(&slot_storage, OuterIndex::new(17));
        remover.remove_cached_index();

        remover.find_outer_index(&slot_storage, OuterIndex::new(14));
        remover.remove_cached_index();

        assert!(remover.pending_write);
        assert_eq!(remover.cached_outer_index, None);
        assert_eq!(
            remover.cache,
            vec![OuterIndex::new(16), OuterIndex::new(15)]
        );
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
    }

    #[test]
    fn test_remove_multiple_different_non_adjacent_slots() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 34;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };
        let list_key_2 = ListKey { index: 2, side };
        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        let mut list_slot_2 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        list_slot_2.inner = [33, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);
        list_slot_2.write_to_slot(&mut slot_storage, &list_key_2);

        remover.find_outer_index(&slot_storage, OuterIndex::new(33));
        remover.remove_cached_index();

        remover.find_outer_index(&slot_storage, OuterIndex::new(14));
        remover.remove_cached_index();

        assert!(remover.pending_write);
        assert_eq!(remover.cached_outer_index, None);
        assert_eq!(
            remover.cache,
            vec![
                OuterIndex::new(34),
                OuterIndex::new(31),
                OuterIndex::new(30),
                OuterIndex::new(29),
                OuterIndex::new(28),
                OuterIndex::new(27),
                OuterIndex::new(26),
                OuterIndex::new(25),
                OuterIndex::new(24),
                OuterIndex::new(23),
                OuterIndex::new(22),
                OuterIndex::new(21),
                OuterIndex::new(20),
                OuterIndex::new(19),
                OuterIndex::new(18),
                OuterIndex::new(17),
                OuterIndex::new(16),
                OuterIndex::new(15)
            ]
        );
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
    }

    #[test]
    fn test_remove_same_slot_ghost_value_from_no_write_case() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 2;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.set(1, OuterIndex::new(200));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        remover.find_outer_index(&slot_storage, OuterIndex::new(200));
        remover.remove_cached_index();
        // No need of a write since we only removed the outermost value
        assert!(!remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        // Ghost value 200 at i = 1 due to no write case
        assert_eq!(
            vec![100, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot.inner
        );
    }

    #[test]
    fn test_remove_same_slot_ghost_value_from_no_write_case_multiple_slots() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 17;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };
        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        remover.find_outer_index(&slot_storage, OuterIndex::new(16));
        remover.remove_cached_index();
        // No need of a write since we only removed the outermost value
        assert!(!remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let read_list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            read_list_slot_0.inner
        );
        // Ghost value at i=0 because of no write case
        assert_eq!(
            vec![16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot_1.inner
        );
    }

    #[test]
    fn test_remove_same_slot_ghost_value_from_write_case() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 2;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(100));
        list_slot.set(1, OuterIndex::new(200));
        list_slot.write_to_slot(&mut slot_storage, &list_key);

        remover.find_outer_index(&slot_storage, OuterIndex::new(100));
        remover.remove_cached_index();
        // We need to write because cache was non-empty
        assert!(remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        // Ghost value 200 at i = 1 due to same slot write case
        assert_eq!(
            vec![200, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot.inner
        );
    }

    #[test]
    fn test_remove_different_slot_no_ghost_value() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 18;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        remover.find_outer_index(&slot_storage, OuterIndex::new(15));
        remover.remove_cached_index();
        // We need to write because cache was non-empty
        assert!(remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let read_list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16],
            read_list_slot_0.inner
        );
        assert_eq!(
            vec![17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot_1.inner
        );
    }

    #[test]
    fn test_remove_from_different_slot_ghost_value_due_to_cleared_slot() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 17;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        remover.find_outer_index(&slot_storage, OuterIndex::new(15));
        remover.remove_cached_index();
        // We need to write because cache was non-empty
        assert!(remover.pending_write);

        remover.write_index_list(&mut slot_storage);
        let read_list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let read_list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16],
            read_list_slot_0.inner
        );
        // Ghost value because slot was cleared
        assert_eq!(
            vec![16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot_1.inner
        );
    }

    #[test]
    fn test_two_types_of_ghost_values_due_to_cached_slot_and_cleared_slot() {
        let mut slot_storage = ArbContext::new();
        let side = Side::Bid;
        let outer_index_count = 17;

        let mut remover = OuterIndexRemover::new(side, outer_index_count);

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let mut list_slot_0 = ListSlot::default();
        let mut list_slot_1 = ListSlot::default();
        list_slot_0.inner = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        list_slot_1.inner = [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);
        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        remover.find_outer_index(&slot_storage, OuterIndex::new(14));
        remover.remove_cached_index();
        remover.find_outer_index(&slot_storage, OuterIndex::new(13));
        remover.remove_cached_index();

        // We need to write because cache was non-empty
        assert!(remover.pending_write);
        remover.write_index_list(&mut slot_storage);

        let read_list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        let read_list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        // Ghost value due to cached slot
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 16, 15],
            read_list_slot_0.inner
        );
        // Ghost value because slot was cleared
        assert_eq!(
            vec![16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            read_list_slot_1.inner
        );
    }
}
