use crate::state::{
    remove::IOuterIndexRemover, write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};
use alloc::vec::Vec;

pub trait IOuterIndexLookupRemover<'a>: IOuterIndexRemover<'a> {
    fn cached_outer_indices_mut(&mut self) -> &mut Vec<OuterIndex>;

    /// Tries to find the outer index in the index list. If the outer index
    /// is found, it is loaded in outer_index_remover.
    ///
    /// Externally ensure that outer indices are sorted in an order moving
    /// away from the centre, i.e. descending for bids and ascending for asks.
    /// This order is enforced by RandomOrderRemover
    ///
    fn find(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        if let Some(outer_index) = self.current_outer_index_mut().take() {
            self.cached_outer_indices_mut().push(outer_index);
        }

        loop {
            if let Some(read_outer_index) = self.active_outer_index_iterator_mut().next(ctx) {
                if read_outer_index == outer_index {
                    *self.current_outer_index_mut() = Some(read_outer_index);
                    return true;
                } else {
                    self.cached_outer_indices_mut().push(read_outer_index);
                }
            } else {
                return false;
            }
        }
    }

    /// Remove, i.e. deactivate the currently cached outer index
    fn remove(&mut self) {
        *self.current_outer_index_mut() = None;
    }

    /// Writes cached outer indices to slot and updates the total outer index count
    ///
    /// If cached outer index exists, increment the outer index count. No
    /// need to push this value to the cached list. This is because the
    /// cached outer index is the current outermost value in the index list.
    fn commit(&mut self, ctx: &mut ArbContext) {
        let list_slot = self.active_outer_index_iterator().list_slot;
        let cached_count = self.cached_outer_indices_mut().len() as u16;

        let outer_index_present = self.current_outer_index().is_some();
        let outer_index_count = self.outer_index_count() + u16::from(outer_index_present);
        write_index_list(
            ctx,
            self.side(),
            self.cached_outer_indices_mut(),
            outer_index_count,
            list_slot,
        );

        // Increase count to account for values written from cache
        self.set_outer_index_count(outer_index_count + cached_count);
    }

    // Setters

    fn set_outer_index_count(&mut self, new_count: u16) {
        *self
            .active_outer_index_iterator_mut()
            .outer_index_count_mut() = new_count;
    }

    // Getters

    fn side(&self) -> Side {
        self.active_outer_index_iterator().side
    }

    fn outer_index_count(&self) -> u16 {
        self.active_outer_index_iterator().outer_index_count()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{
        remove::OuterIndexLookupRemover, ArbContext, ContextActions, ListKey, ListSlot, OuterIndex,
        Side,
    };

    use super::IOuterIndexLookupRemover;

    #[test]
    fn test_lookup_across_list_items_for_bids() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 19;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_item_1 = ListSlot {
            inner: [16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexLookupRemover::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(18);
        remover.find(ctx, outer_index_0);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_0);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 18);
        assert_eq!(remover.cached_outer_indices, vec![]);

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 18);
        assert_eq!(remover.cached_outer_indices, vec![]);

        let outer_index_1 = OuterIndex::new(16);
        remover.find(ctx, outer_index_1);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_1);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 16);
        assert_eq!(remover.cached_outer_indices, vec![OuterIndex::new(17)]);

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 16);
        assert_eq!(remover.cached_outer_indices, vec![OuterIndex::new(17)]);

        // Remove in different group

        let outer_index_2 = OuterIndex::new(14);
        remover.find(ctx, outer_index_2);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_2);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![OuterIndex::new(17), OuterIndex::new(15)]
        );

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![OuterIndex::new(17), OuterIndex::new(15)]
        );

        remover.commit(ctx);

        // Cache written to list slot 0
        let read_list_item_0 = ListSlot::new_from_slot(ctx, list_key_0);
        assert_eq!(
            read_list_item_0,
            ListSlot {
                inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 17]
            }
        );

        // Holds garbage values because slot was closed
        let read_list_item_1 = ListSlot::new_from_slot(ctx, list_key_1);
        assert_eq!(read_list_item_1, list_item_1);
    }

    #[test]
    fn test_lookup_across_list_items_for_asks() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 19;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3],
        };
        let list_item_1 = ListSlot {
            inner: [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexLookupRemover::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(0);
        remover.find(ctx, outer_index_0);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_0);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 18);
        assert_eq!(remover.cached_outer_indices, vec![]);

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 18);
        assert_eq!(remover.cached_outer_indices, vec![]);

        let outer_index_1 = OuterIndex::new(2);
        remover.find(ctx, outer_index_1);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_1);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 16);
        assert_eq!(remover.cached_outer_indices, vec![OuterIndex::new(1)]);

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 16);
        assert_eq!(remover.cached_outer_indices, vec![OuterIndex::new(1)]);

        // Remove in different group

        let outer_index_2 = OuterIndex::new(4);
        remover.find(ctx, outer_index_2);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_2);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![OuterIndex::new(1), OuterIndex::new(3)]
        );

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![OuterIndex::new(1), OuterIndex::new(3)]
        );
    }

    #[test]
    fn test_lookup_one_but_remove_another_for_bids() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 19;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_item_1 = ListSlot {
            inner: [16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexLookupRemover::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(18);
        remover.find(ctx, outer_index_0);

        let outer_index_1 = OuterIndex::new(14);
        remover.find(ctx, outer_index_1);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_1);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![
                OuterIndex::new(18),
                OuterIndex::new(17),
                OuterIndex::new(16),
                OuterIndex::new(15)
            ]
        );

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![
                OuterIndex::new(18),
                OuterIndex::new(17),
                OuterIndex::new(16),
                OuterIndex::new(15)
            ]
        );
    }

    #[test]
    fn test_lookup_one_but_remove_another_for_asks() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 19;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3],
        };
        let list_item_1 = ListSlot {
            inner: [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexLookupRemover::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(0);
        remover.find(ctx, outer_index_0);

        let outer_index_1 = OuterIndex::new(4);
        remover.find(ctx, outer_index_1);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_1);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![
                OuterIndex::new(0),
                OuterIndex::new(1),
                OuterIndex::new(2),
                OuterIndex::new(3)
            ]
        );

        remover.remove();
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 14);
        assert_eq!(
            remover.cached_outer_indices,
            vec![
                OuterIndex::new(0),
                OuterIndex::new(1),
                OuterIndex::new(2),
                OuterIndex::new(3)
            ]
        );
    }

    #[test]
    fn test_looked_up_item_is_committed_back() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 19;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_item_1 = ListSlot {
            inner: [16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexLookupRemover::new(side, &mut outer_index_count);

        let outer_index_0 = OuterIndex::new(12);
        remover.find(ctx, outer_index_0);
        remover.remove();

        // Find but not remove
        let outer_index_1 = OuterIndex::new(10);
        remover.find(ctx, outer_index_1);
        assert_eq!(remover.current_outer_index.unwrap(), outer_index_1);
        assert_eq!(remover.active_outer_index_iterator.outer_index_count(), 10);
        assert_eq!(
            remover.cached_outer_indices,
            vec![
                OuterIndex::new(18),
                OuterIndex::new(17),
                OuterIndex::new(16),
                OuterIndex::new(15),
                OuterIndex::new(14),
                OuterIndex::new(13),
                OuterIndex::new(11),
            ]
        );

        remover.commit(ctx);

        // Cache written to list slot 0
        let read_list_item_0 = ListSlot::new_from_slot(ctx, list_key_0);
        assert_eq!(
            read_list_item_0,
            ListSlot {
                inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16]
            }
        );

        // Holds garbage values because slot was closed
        let read_list_item_1 = ListSlot::new_from_slot(ctx, list_key_1);
        assert_eq!(
            read_list_item_1,
            ListSlot {
                inner: [17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
        );
    }
}
