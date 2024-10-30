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

    /// Prepare an outer index for insertion in the index list
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
    pub fn write_index_list(&mut self, ctx: &mut ArbContext) {
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
