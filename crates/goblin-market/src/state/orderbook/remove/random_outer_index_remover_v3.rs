use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};

use alloc::vec::Vec;

use super::sequential_outer_index_remover_v3::ISequentialOuterIndexRemover;

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

pub trait IRandomOuterIndexRemover<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a>;
    fn current_outer_index(&mut self) -> &mut Option<OuterIndex>;
    fn cached_outer_indices(&mut self) -> &mut Vec<OuterIndex>;

    /// Tries to find the outer index in the index list. If the outer index
    /// is found, it is loaded in outer_index_remover.
    ///
    /// Externally ensure that outer indices are sorted in an order moving
    /// away from the centre, i.e. descending for bids and ascending for asks.
    /// This order is enforced by RandomOrderRemover
    ///
    fn find(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if let Some(read_outer_index) = self.active_outer_index_iterator().next(ctx) {
                if read_outer_index == outer_index {
                    *self.current_outer_index() = Some(read_outer_index);
                    return true;
                } else {
                    self.cached_outer_indices().push(read_outer_index);
                }
            } else {
                return false;
            }
        }
    }

    /// Remove, i.e. deactivate the currently cached outer index
    fn remove(&mut self) {
        *self.current_outer_index() = None;
    }

    /// Writes cached outer indices to slot and updates the total outer index count
    ///
    /// If cached outer index exists, increment the outer index count. No
    /// need to push this value to the cached list. This is because the
    /// cached outer index is the current outermost value in the index list.
    fn commit_outer_index_remover(&mut self, ctx: &mut ArbContext) {
        let list_slot = self.active_outer_index_iterator().list_slot;
        let cached_count = self.cached_outer_indices().len() as u16;

        let outer_index_present = self.current_outer_index().is_some();
        let outer_index_count = self.outer_index_count() + u16::from(outer_index_present);

        write_index_list(
            ctx,
            self.side(),
            self.cached_outer_indices(),
            outer_index_count,
            list_slot,
        );

        // Increase count to account for values written from cache
        self.set_outer_index_count(outer_index_count + cached_count);
    }

    fn side(&mut self) -> Side {
        self.active_outer_index_iterator().side
    }

    fn outer_index_count(&mut self) -> u16 {
        *self.active_outer_index_iterator().outer_index_count_mut()
    }

    fn set_outer_index_count(&mut self, count: u16) {
        *self.active_outer_index_iterator().outer_index_count_mut() = count;
    }
}

impl<'a> IRandomOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }

    fn cached_outer_indices(&mut self) -> &mut Vec<OuterIndex> {
        &mut self.cached_outer_indices
    }
}

impl<'a> ISequentialOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}
