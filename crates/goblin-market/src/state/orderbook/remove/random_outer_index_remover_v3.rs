use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};

use alloc::vec::Vec;

use super::sequential_outer_index_remover_v3::{IOuterIndexRemover, ISequentialOuterIndexRemover};

/// Lookup and delete remove arbitrary outer indices. The outer
/// indices must be sorted in a direction moving away from centre of the book
pub struct RandomOuterIndexRemoverV3<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,

    /// Outer indices that need to be written back to the list
    pub cached_outer_indices: Vec<OuterIndex>,
}

impl<'a> RandomOuterIndexRemoverV3<'a> {
    /// Constructs a new RandomOuterIndexRemover
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
}

pub trait IRandomOuterIndexRemover<'a>: IOuterIndexRemover<'a> {
    fn cached_outer_indices_mut(&mut self) -> &mut Vec<OuterIndex>;

    /// Tries to find the outer index in the index list. If the outer index
    /// is found, it is loaded in outer_index_remover.
    ///
    /// Externally ensure that outer indices are sorted in an order moving
    /// away from the centre, i.e. descending for bids and ascending for asks.
    /// This order is enforced by RandomOrderRemover
    ///
    fn find(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
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
    fn commit_outer_index_remover(&'a mut self, ctx: &mut ArbContext) {
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

impl<'a> IRandomOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
    fn cached_outer_indices_mut(&mut self) -> &mut Vec<OuterIndex> {
        &mut self.cached_outer_indices
    }
}

impl<'a> IOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
    fn active_outer_index_iterator(&self) -> &ActiveOuterIndexIteratorV2<'a> {
        &self.active_outer_index_iterator
    }

    fn active_outer_index_iterator_mut(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&self) -> Option<OuterIndex> {
        self.current_outer_index
    }

    fn current_outer_index_mut(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}

impl<'a> ISequentialOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {}
