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

    /// The currently read outer index
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

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
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
    pub fn insert_if_absent(&mut self, ctx: &ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if let Some(current_outer_index) = self.current_outer_index {
                if current_outer_index == outer_index
                // || self
                //     .cached_outer_indices
                //     .last()
                //     .is_some_and(|last_cached_outer_index| {
                //         *last_cached_outer_index == outer_index
                //     })
                {
                    // value found, no need to insert
                    return false;
                } else if self.side() == Side::Bid && outer_index > current_outer_index
                    || self.side() == Side::Ask && outer_index < current_outer_index
                {
                    // alt design- by storing the incoming value in current_outer_index()
                    // this function can automatically filter out duplicate outer indices.
                    // In the previous design, we do not call insert_if_absent() if the
                    // value was present already. This requires us to store the previous
                    // outer index in a separate variable.
                    //
                    // value inserted
                    self.cached_outer_indices.push(outer_index);
                    return true;
                } else {
                    // need to look deeper. Push current value to cache and continue looking
                    self.current_outer_index = None;
                    self.cached_outer_indices.push(current_outer_index);
                }
            }

            if let Some(next_outer_index) = self.active_outer_index_iterator.next(ctx) {
                self.current_outer_index = Some(next_outer_index);
            } else {
                // Alt design- current_outer_index should only hold the last read value.
                // This way we can write it back by just incrementing the index count.
                self.cached_outer_indices.push(outer_index);
                return true;
            }
        }
    }

    /// Number of outer indices yet to be read plus the cached index if present
    fn remaining_outer_indices(&self) -> u16 {
        let outer_index_present = self.current_outer_index.is_some();
        self.active_outer_index_iterator.unread_outer_indices() + u16::from(outer_index_present)
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
        // assert_eq!(inserter.current_outer_index, None);
        // assert_eq!(inserter.cached_outer_indices, vec![outer_index_0]);
        // assert_eq!(
        //     inserter.active_outer_index_iterator.unread_outer_indices(),
        //     0
        // );

        // // Insert next
        // inserter.insert_if_absent(ctx, outer_index_1);
        // assert_eq!(inserter.current_outer_index, None);
        // assert_eq!(
        //     inserter.cached_outer_indices,
        //     vec![outer_index_0, outer_index_1]
        // );
        // assert_eq!(
        //     inserter.active_outer_index_iterator.unread_outer_indices(),
        //     0
        // );

        // // Commit
        // inserter.commit(ctx);
    }
}

// #[cfg(test)]
// mod tests_old {
//     use crate::state::{ContextActions, ListKey, ListSlot};

//     use super::*;

//     #[test]
//     fn test_prepare_bid_empty_list() {
//         let ctx = &mut ArbContext::new();
//         let mut outer_index_count = 0;
//         let mut insertion = OuterIndexInserterV2::new(Side::Bid, &mut outer_index_count);

//         // Insert into an empty list
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);

//         // Insert duplicate
//         assert!(!insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);

//         // Insert an index closer to the center
//         // Externally ensure that subsequent indices move away from the centre.
//         // This case is to deal with the last value from .next()
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(150)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(150), OuterIndex::new(100)]
//         );

//         // Insert an index further away from the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(50)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![
//                 OuterIndex::new(150),
//                 OuterIndex::new(100),
//                 OuterIndex::new(50)
//             ]
//         );
//     }

//     #[test]
//     fn test_prepare_bid_equal_index() {
//         let mut ctx = &mut ArbContext::new();

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(
//                 &mut ctx,
//                 &ListKey {
//                     index: 0,
//                     side: Side::Bid,
//                 },
//             );
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(Side::Bid, &mut outer_index_count);

//         // Attempt to insert the same index
//         assert!(!insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);
//     }

//     #[test]
//     fn test_prepare_bid_closer_to_center() {
//         let ctx = &mut ArbContext::new();

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(
//                 ctx,
//                 &ListKey {
//                     index: 0,
//                     side: Side::Bid,
//                 },
//             );
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(Side::Bid, &mut outer_index_count);

//         // Insert an index closer to the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(150)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(150), OuterIndex::new(100)]
//         );
//     }

//     #[test]
//     fn test_prepare_bid_away_from_center() {
//         let mut ctx = &mut ArbContext::new();

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(
//                 &mut ctx,
//                 &ListKey {
//                     index: 0,
//                     side: Side::Bid,
//                 },
//             );
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(Side::Bid, &mut outer_index_count);

//         // Insert an index further away from the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(50)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(100), OuterIndex::new(50)]
//         );
//     }

//     #[test]
//     fn test_prepare_ask_empty_list() {
//         let ctx = &mut ArbContext::new();
//         let mut outer_index_count = 0;
//         let mut insertion = OuterIndexInserterV2::new(Side::Ask, &mut outer_index_count);

//         // Insert into an empty list
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);

//         // Insert duplicate
//         assert!(!insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);

//         // Insert an index closer to the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(50)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(50), OuterIndex::new(100)]
//         );

//         // Insert an index further away from the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(150)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![
//                 OuterIndex::new(50),
//                 OuterIndex::new(100),
//                 OuterIndex::new(150)
//             ]
//         );
//     }

//     #[test]
//     fn test_prepare_ask_equal_index() {
//         let ctx = &mut ArbContext::new();
//         let side = Side::Ask;

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(ctx, &ListKey { index: 0, side });
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(side, &mut outer_index_count);

//         // Attempt to insert the same index
//         assert!(!insertion.insert_if_absent(ctx, OuterIndex::new(100)));
//         assert_eq!(insertion.cached_outer_indices, vec![OuterIndex::new(100)]);
//     }

//     #[test]
//     fn test_prepare_ask_closer_to_center() {
//         let ctx = &mut ArbContext::new();
//         let side = Side::Ask;

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(ctx, &ListKey { index: 0, side });
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(side, &mut outer_index_count);

//         // Insert an index closer to the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(50)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(50), OuterIndex::new(100)]
//         );
//     }

//     #[test]
//     fn test_prepare_ask_away_from_center() {
//         let ctx = &mut ArbContext::new();
//         let side = Side::Ask;

//         // Setup the initial slot storage with one item
//         {
//             let mut list_slot = ListSlot::default();
//             list_slot.set(0, OuterIndex::new(100));
//             list_slot.write_to_slot(ctx, &ListKey { index: 0, side });
//         }

//         let mut outer_index_count = 1;
//         let mut insertion = OuterIndexInserterV2::new(side, &mut outer_index_count);

//         // Insert an index further away from the center
//         assert!(insertion.insert_if_absent(ctx, OuterIndex::new(150)));
//         assert_eq!(
//             insertion.cached_outer_indices,
//             vec![OuterIndex::new(100), OuterIndex::new(150)]
//         );
//     }
// }
