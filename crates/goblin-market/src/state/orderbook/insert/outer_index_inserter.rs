use crate::state::{
    iterator::active_position::active_outer_index_iterator::ActiveOuterIndexIterator,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};
use alloc::vec::Vec;

/// Enables bulk insertion of outer indices in the index list.
/// Successive inserted orders should move away from the centre, i.e.
///
/// - insert bids in descending order
/// - insert asks in ascending order
///
pub struct OuterIndexInserter {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIterator,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be inserted and values popped from index list reader
    /// in the correct order of insertion.
    pub cache: Vec<OuterIndex>,
}

impl OuterIndexInserter {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIterator::new(side, outer_index_count),
            cache: Vec::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }

    /// Prepare an outer index for insertion in the index list
    ///
    /// # Arguments
    ///
    /// * outer_index
    /// * slot_storage
    ///
    /// # Returns
    ///
    /// Returns true if the value needs insertion, false if it is already present
    ///
    pub fn insert_if_absent(&mut self, slot_storage: &ArbContext, outer_index: OuterIndex) -> bool {
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
        while let Some(current_outer_index) = self.active_outer_index_iterator.next(slot_storage) {
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
    pub fn write_index_list(&mut self, slot_storage: &mut ArbContext) {
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
    fn test_prepare_bid_empty_list() {
        let slot_storage = &mut ArbContext::new();
        let mut insertion = OuterIndexInserter::new(Side::Bid, 0);

        // Insert into an empty list
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        // Externally ensure that subsequent indices move away from the centre.
        // This case is to deal with the last value from .next()
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(50)));
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
        let mut slot_storage = &mut ArbContext::new();

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

        let mut insertion = OuterIndexInserter::new(Side::Bid, 1);

        // Attempt to insert the same index
        assert!(!insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_bid_closer_to_center() {
        let slot_storage = &mut ArbContext::new();

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

        let mut insertion = OuterIndexInserter::new(Side::Bid, 1);

        // Insert an index closer to the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_bid_away_from_center() {
        let mut slot_storage = &mut ArbContext::new();

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

        let mut insertion = OuterIndexInserter::new(Side::Bid, 1);

        // Insert an index further away from the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(50)]
        );
    }

    #[test]
    fn test_prepare_ask_empty_list() {
        let slot_storage = &mut ArbContext::new();
        let mut insertion = OuterIndexInserter::new(Side::Ask, 0);

        // Insert into an empty list
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(150)));
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
        let slot_storage = &mut ArbContext::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = OuterIndexInserter::new(side, 1);

        // Attempt to insert the same index
        assert!(!insertion.insert_if_absent(slot_storage, OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_ask_closer_to_center() {
        let slot_storage = &mut ArbContext::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = OuterIndexInserter::new(side, 1);

        // Insert an index closer to the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_ask_away_from_center() {
        let slot_storage = &mut ArbContext::new();
        let side = Side::Ask;

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(slot_storage, &ListKey { index: 0, side });
        }

        let mut insertion = OuterIndexInserter::new(side, 1);

        // Insert an index further away from the center
        assert!(insertion.insert_if_absent(slot_storage, OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(150)]
        );
    }
}
