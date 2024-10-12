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
