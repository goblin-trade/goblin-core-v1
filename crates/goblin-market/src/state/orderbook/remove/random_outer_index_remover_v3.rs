use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    write_index_list::write_index_list, ArbContext, OuterIndex,
};

use alloc::vec::Vec;

use super::sequential_outer_index_remover_v3::ISequentialOuterIndexRemover;

pub struct RandomOuterIndexRemoverV3<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,

    pub cached_outer_indices: Vec<OuterIndex>,
}

impl<'a> ISequentialOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}

impl<'a> RandomOuterIndexRemoverV3<'a> {
    /// Tries to find the outer index in the index list. If the outer index
    /// is found, it is loaded in outer_index_remover.
    ///
    /// Externally ensure that outer indices are sorted in an order moving
    /// away from the centre, i.e. descending for bids and ascending for asks.
    /// This order is enforced by RandomOrderRemover
    ///
    pub fn find_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        loop {
            if let Some(read_outer_index) = self.active_outer_index_iterator.next(ctx) {
                if read_outer_index == outer_index {
                    self.current_outer_index = Some(read_outer_index);
                    return true;
                } else {
                    self.cached_outer_indices.push(read_outer_index);
                }
            } else {
                return false;
            }
        }
    }

    /// Writes cached outer indices to slot and updates the total outer index count
    ///
    /// If cached outer index exists, increment the outer index count. No
    /// need to push this value to the cached list. This is because the
    /// cached outer index is the current outermost value in the index list.
    pub fn commit_outer_index_remover(
        &mut self,
        ctx: &mut ArbContext,
        // outer_index_remover: &mut SequentialOuterIndexRemover,
        // cached_outer_indices: &mut Vec<OuterIndex>,
    ) {
        // let side = outer_index_remover.side();
        let side = self.active_outer_index_iterator.side;
        let list_slot = self.active_outer_index_iterator.list_slot;
        let cached_count = self.cached_outer_indices.len() as u16;

        let outer_index_count = self.active_outer_index_iterator.outer_index_count_mut();
        *outer_index_count += u16::from(self.current_outer_index.is_some());

        write_index_list(
            ctx,
            side,
            &mut self.cached_outer_indices,
            *outer_index_count,
            list_slot,
        );

        // Increase count to account for values written from cache
        *outer_index_count += cached_count;
    }
}
