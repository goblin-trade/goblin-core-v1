use alloc::vec::Vec;

use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};

/// Enables bulk insertion of outer indices in the index list.
/// Successive inserted orders should move away from the centre, i.e.
///
/// - insert bids in descending order
/// - insert asks in ascending order
///
pub struct OuterIndexInserterV2<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// Cached active outer indices which will be written back to slots.
    pub cached_outer_indices: Vec<OuterIndex>,

    /// The currently read outer index from the index list.
    /// This does not hold inserted values.
    pub current_outer_index: Option<OuterIndex>,
}

impl<'a> OuterIndexInserterV2<'a> {
    /// Constructs a new OuterIndexInserter
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `outer_index_count` - Reference to outer index count for the given
    /// side in MarketState
    pub fn new(side: Side, outer_index_count: &'a mut u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIteratorV2::new(side, outer_index_count),
            current_outer_index: None,
            cached_outer_indices: Vec::new(),
        }
    }

    /// Prepare an outer index for insertion in the index list. Insertions are always
    /// successful. This function returns true if the value was inserted and false
    /// if it was already present.
    ///
    /// The result is used to decide whether a group position should be read or
    /// initialized with 0.
    ///
    /// # Arguments
    ///
    /// * outer_index
    /// * ctx
    ///
    /// # Returns
    ///
    /// Returns true if the value needs insertion, false if it is already present
    ///
    pub fn insert_if_absent_old(&mut self, ctx: &ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if let Some(current_outer_index) = self.current_outer_index {
                if current_outer_index == outer_index {
                    // value found, no need to insert
                    return false;
                } else if self.side() == Side::Bid && outer_index > current_outer_index
                    || self.side() == Side::Ask && outer_index < current_outer_index
                {
                    // value inserted
                    self.cached_outer_indices.push(outer_index);
                    return true;
                } else {
                    // need to look deeper. Push current value to cache and continue looking
                    self.cached_outer_indices.push(current_outer_index);
                    self.current_outer_index = None;
                }
            }

            if let Some(next_outer_index) = self.active_outer_index_iterator.next(ctx) {
                self.current_outer_index = Some(next_outer_index);
            } else if self
                .cached_outer_indices
                .last()
                .is_some_and(|last_cached_outer_index| *last_cached_outer_index == outer_index)
            {
                // Avoid writing duplicate values
                return false;
            } else {
                self.cached_outer_indices.push(outer_index);
                return true;
            }
        }
    }

    pub fn insert_if_absent(&mut self, ctx: &ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if self
                .last_added_outer_index()
                .is_some_and(|last_outer_index| last_outer_index == outer_index)
            {
                return false;
            } else if let Some(current_outer_index) = self.current_outer_index {
                if outer_index.is_closer_to_center(self.side(), current_outer_index) {
                    // value inserted
                    self.cached_outer_indices.push(outer_index);
                    return true;
                } else {
                    // need to look deeper. Push current value to cache and continue looking
                    self.cached_outer_indices.push(current_outer_index);
                    self.current_outer_index = None;
                }
            } else if let Some(next_outer_index) = self.active_outer_index_iterator.next(ctx) {
                // Read next item from index list
                self.current_outer_index = Some(next_outer_index);
            } else {
                // If index list is exhausted save the outer index and exit
                self.cached_outer_indices.push(outer_index);
                return true;
            }
        }
    }

    /// Write prepared indices to slot
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        let list_slot = self.active_outer_index_iterator.list_slot;
        let cached_count = self.cached_outer_indices.len() as u16;
        let remaining_outer_indices = self.remaining_outer_indices();

        write_index_list(
            ctx,
            self.side(),
            &mut self.cached_outer_indices,
            remaining_outer_indices,
            list_slot,
        );
        // Increase count to account for values written from cache
        self.set_unread_outer_indices(remaining_outer_indices + cached_count);
    }

    // Setters
    fn set_unread_outer_indices(&mut self, new_count: u16) {
        *self.active_outer_index_iterator.unread_outer_indices_mut() = new_count;
    }

    // Getters

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }

    pub fn last_added_outer_index(&self) -> Option<OuterIndex> {
        if self.current_outer_index.is_some() {
            self.current_outer_index
        } else {
            self.cached_outer_indices.last().map(|last| *last)
        }
    }

    /// Number of outer indices yet to be read plus the last cached index if present
    fn remaining_outer_indices(&self) -> u16 {
        let outer_index_present = self.current_outer_index.is_some();
        self.active_outer_index_iterator.unread_outer_indices() + u16::from(outer_index_present)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ContextActions, ListKey, ListSlot};

    #[test]
    fn test_insert_bids_in_empty_list() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let mut outer_index_count = 0;
        let mut inserter = OuterIndexInserterV2::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(3);
        let outer_index_1 = OuterIndex::new(2);

        // Insert first
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_0), true);
        assert_eq!(inserter.current_outer_index, None);
        assert_eq!(inserter.cached_outer_indices, vec![outer_index_0]);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Insert duplicate- no effect
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_0), false);
        assert_eq!(inserter.current_outer_index, None);
        assert_eq!(inserter.cached_outer_indices, vec![outer_index_0]);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Insert next
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_1), true);
        assert_eq!(inserter.current_outer_index, None);
        assert_eq!(
            inserter.cached_outer_indices,
            vec![outer_index_0, outer_index_1]
        );
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Commit
        inserter.commit(ctx);
        assert_eq!(inserter.current_outer_index, None);
        assert_eq!(inserter.cached_outer_indices, vec![]);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            2
        );

        let mut expected_list_slot_0 = ListSlot::default();
        expected_list_slot_0.set(0, outer_index_1);
        expected_list_slot_0.set(1, outer_index_0);

        let read_list_slot_0 = ListSlot::new_from_slot(ctx, ListKey { index: 0, side });
        assert_eq!(read_list_slot_0, expected_list_slot_0);
    }

    #[test]
    fn test_insert_bid_closer_to_centre() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let mut outer_index_count = 1;
        let outer_index_0 = OuterIndex::new(2);

        let list_key_0 = ListKey { index: 0, side };
        let mut list_slot_0 = ListSlot::default();
        list_slot_0.set(0, outer_index_0);
        list_slot_0.write_to_slot(ctx, &list_key_0);

        let mut inserter = OuterIndexInserterV2::new(side, &mut outer_index_count);

        let outer_index_to_insert = OuterIndex::new(3);
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_to_insert), true);

        assert_eq!(inserter.current_outer_index.unwrap(), outer_index_0);
        assert_eq!(inserter.cached_outer_indices, vec![outer_index_to_insert]);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Commit
        inserter.commit(ctx);

        let mut expected_list_slot_0 = ListSlot::default();
        expected_list_slot_0.set(0, outer_index_0);
        expected_list_slot_0.set(1, outer_index_to_insert);

        let read_list_slot_0 = ListSlot::new_from_slot(ctx, list_key_0);
        assert_eq!(read_list_slot_0, expected_list_slot_0);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            2
        );
    }

    #[test]
    fn test_insert_bid_equal_to_stored_value() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let mut outer_index_count = 1;
        let outer_index_0 = OuterIndex::new(2);

        let list_key_0 = ListKey { index: 0, side };
        let mut list_slot_0 = ListSlot::default();
        list_slot_0.set(0, outer_index_0);
        list_slot_0.write_to_slot(ctx, &list_key_0);

        let mut inserter = OuterIndexInserterV2::new(side, &mut outer_index_count);

        // Try to insert duplicate
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_0), false);
        assert_eq!(inserter.current_outer_index.unwrap(), outer_index_0);
        assert_eq!(inserter.cached_outer_indices, vec![]);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Commit
        inserter.commit(ctx);

        let read_list_slot_0 = ListSlot::new_from_slot(ctx, list_key_0);
        assert_eq!(read_list_slot_0, list_slot_0);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            1
        );
    }

    #[test]
    fn test_insert_bid_further_from_centre() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let mut outer_index_count = 1;
        let outer_index_0 = OuterIndex::new(2);

        let list_key_0 = ListKey { index: 0, side };
        let mut list_slot_0 = ListSlot::default();
        list_slot_0.set(0, outer_index_0);
        list_slot_0.write_to_slot(ctx, &list_key_0);

        let mut inserter = OuterIndexInserterV2::new(side, &mut outer_index_count);

        let outer_index_to_insert = OuterIndex::new(1);
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_to_insert), true);
        assert_eq!(inserter.current_outer_index, None);
        assert_eq!(
            inserter.cached_outer_indices,
            vec![outer_index_0, outer_index_to_insert]
        );
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // Commit
        inserter.commit(ctx);

        let mut expected_list_slot_0 = ListSlot::default();
        expected_list_slot_0.set(0, outer_index_to_insert);
        expected_list_slot_0.set(1, outer_index_0);

        let read_list_slot_0 = ListSlot::new_from_slot(ctx, list_key_0);
        assert_eq!(read_list_slot_0, expected_list_slot_0);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            2
        );
    }

    #[test]
    fn test_insert_bid_leads_to_write_on_new_slot() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let mut outer_index_count = 16;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };
        let list_slot_0 = ListSlot {
            inner: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
        };
        list_slot_0.write_to_slot(ctx, &list_key_0);

        let mut inserter = OuterIndexInserterV2::new(side, &mut outer_index_count);

        let outer_index_to_insert = OuterIndex::new(16);
        assert_eq!(inserter.insert_if_absent(ctx, outer_index_to_insert), true);
        assert_eq!(inserter.current_outer_index.unwrap(), OuterIndex::new(15));
        assert_eq!(
            inserter.cached_outer_indices,
            vec![OuterIndex::new(17), outer_index_to_insert]
        );
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            14
        );

        // Commit
        inserter.commit(ctx);

        let expected_list_slot_0 = ListSlot {
            inner: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        };
        let expected_list_slot_1 = ListSlot {
            inner: [17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        let read_list_slot_0 = ListSlot::new_from_slot(ctx, list_key_0);
        let read_list_slot_1 = ListSlot::new_from_slot(ctx, list_key_1);
        assert_eq!(read_list_slot_0, expected_list_slot_0);
        assert_eq!(read_list_slot_1, expected_list_slot_1);
        assert_eq!(
            inserter.active_outer_index_iterator.unread_outer_indices(),
            17
        );
    }
}
