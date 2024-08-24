use super::{IndexListIterator, ListKey, ListSlot, OuterIndex};
use crate::state::{Side, SlotStorage};
use alloc::vec::Vec;

pub struct IndexListInsertion<'a> {
    pub index_list_iterator: IndexListIterator<'a>,
    pub cache: Vec<OuterIndex>, // Cache for storing outer indices
    pub side: Side,             // Indicates whether it's for Bids or Asks
}

impl<'a> IndexListInsertion<'a> {
    pub fn new(side: Side, outer_index_count: u16, slot_storage: &'a mut SlotStorage) -> Self {
        let index_list_iterator = IndexListIterator::new(outer_index_count, slot_storage);

        Self {
            index_list_iterator,
            cache: Vec::new(),
            side,
        }
    }

    pub fn prepare(&mut self, outer_index: OuterIndex) -> bool {
        // Check last element in the cache
        if let Some(&last_pushed_outer_index) = self.cache.last() {
            // If the element already exists in the cache, return false
            if last_pushed_outer_index == outer_index {
                return false;
            }

            // If the last element in cache is closer to the center than outer_index, insert before it
            if last_pushed_outer_index.is_closer_to_center(self.side, outer_index) {
                self.cache.pop(); // Remove the last pushed index
                self.cache.push(outer_index);
                self.cache.push(last_pushed_outer_index); // Push it back after the new index
                return true;
            }
        }

        // Iterate through the list to find the correct position
        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_iterator.next()
        {
            // If the outer_index is already in the list, only insert once
            if current_outer_index == outer_index {
                self.cache.push(current_outer_index);
                return false;
            }

            // If outer_index is closer to the center, insert before current_outer_index
            if current_outer_index.is_closer_to_center(self.side, outer_index) {
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

    /// Write cached indices to slot
    ///
    /// This must be called after calling prepare() atlreast once, else the function fails
    /// because index_list_iterator.list_slot.unwrap() will throw error
    ///
    pub fn commit(&mut self) {
        if self.cache.is_empty() {
            return;
        }

        let start_slot_index = self.index_list_iterator.outer_index_count / 16;
        // let mut start_relative_index = self.index_list_iterator.outer_index_count % 16;

        let size_after_insertions =
            self.index_list_iterator.outer_index_count + self.cache.len() as u16;

        let final_slot_index_inclusive = (size_after_insertions - 1) / 16;

        for slot_index in start_slot_index..=final_slot_index_inclusive {
            let (mut list_slot, start_relative_index) = if slot_index == start_slot_index {
                (
                    self.index_list_iterator.list_slot.unwrap(),
                    self.index_list_iterator.outer_index_count % 16,
                )
            } else {
                (ListSlot::default(), 0)
            };

            let final_relative_index_inclusive = if slot_index == final_slot_index_inclusive - 1 {
                (size_after_insertions - 1) % 16
            } else {
                16
            };

            for relative_index in start_relative_index..=final_relative_index_inclusive {
                list_slot.set(relative_index as usize, self.cache.pop().unwrap());
            }

            list_slot.write_to_slot(
                self.index_list_iterator.slot_storage,
                &ListKey { index: slot_index },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::SlotActions;

    use super::*;

    #[test]
    fn test_prepare_bid_empty_list() {
        let mut slot_storage = SlotStorage::new();
        let mut insertion = IndexListInsertion::new(Side::Bid, 0, &mut slot_storage);

        // Insert into an empty list
        assert!(insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        // Externally ensure that subsequent indices move away from the centre.
        // This case is to deal with the last value from .next()
        assert!(insertion.prepare(OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.prepare(OuterIndex::new(50)));
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
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Bid, 1, &mut slot_storage);

        // Attempt to insert the same index
        assert!(!insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_bid_closer_to_center() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Bid, 1, &mut slot_storage);

        // Insert an index closer to the center
        assert!(insertion.prepare(OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(150), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_bid_away_from_center() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Bid, 1, &mut slot_storage);

        // Insert an index further away from the center
        assert!(insertion.prepare(OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(50)]
        );
    }

    #[test]
    fn test_prepare_ask_empty_list() {
        let mut slot_storage = SlotStorage::new();
        let mut insertion = IndexListInsertion::new(Side::Ask, 0, &mut slot_storage);

        // Insert into an empty list
        assert!(insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert duplicate
        assert!(!insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);

        // Insert an index closer to the center
        assert!(insertion.prepare(OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );

        // Insert an index further away from the center
        assert!(insertion.prepare(OuterIndex::new(150)));
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
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Ask, 1, &mut slot_storage);

        // Attempt to insert the same index
        assert!(!insertion.prepare(OuterIndex::new(100)));
        assert_eq!(insertion.cache, vec![OuterIndex::new(100)]);
    }

    #[test]
    fn test_prepare_ask_closer_to_center() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Ask, 1, &mut slot_storage);

        // Insert an index closer to the center
        assert!(insertion.prepare(OuterIndex::new(50)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(50), OuterIndex::new(100)]
        );
    }

    #[test]
    fn test_prepare_ask_away_from_center() {
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }

        let mut insertion = IndexListInsertion::new(Side::Ask, 1, &mut slot_storage);

        // Insert an index further away from the center
        assert!(insertion.prepare(OuterIndex::new(150)));
        assert_eq!(
            insertion.cache,
            vec![OuterIndex::new(100), OuterIndex::new(150)]
        );
    }
}
