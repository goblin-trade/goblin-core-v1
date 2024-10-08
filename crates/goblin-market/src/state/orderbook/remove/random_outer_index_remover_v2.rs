use crate::state::{write_index_list::write_index_list, ArbContext, OuterIndex};

use super::sequential_outer_index_remover::SequentialOuterIndexRemover;
use alloc::vec::Vec;

/// Tries to find the outer index in the index list. If the outer index
/// is found, it is loaded in outer_index_remover.
///
/// Externally ensure that outer indices are sorted in an order moving
/// away from the centre, i.e. descending for bids and ascending for asks.
/// This order is enforced by RandomOrderRemover
///
pub fn find_outer_index(
    ctx: &mut ArbContext,
    outer_index_remover: &mut SequentialOuterIndexRemover,
    cached_outer_indices: &mut Vec<OuterIndex>,
    outer_index: OuterIndex,
) -> bool {
    loop {
        if let Some(read_outer_index) = outer_index_remover.active_outer_index_iterator.next(ctx) {
            if read_outer_index == outer_index {
                outer_index_remover.current_outer_index = Some(read_outer_index);
                return true;
            } else {
                cached_outer_indices.push(read_outer_index);
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
    ctx: &mut ArbContext,
    outer_index_remover: &mut SequentialOuterIndexRemover,
    cached_outer_indices: &mut Vec<OuterIndex>,
) {
    let side = outer_index_remover.side();
    let list_slot = outer_index_remover.active_outer_index_iterator.list_slot;
    let cached_count = cached_outer_indices.len() as u16;

    outer_index_remover.commit(); // Adds 1 if current_outer_index is present

    let outer_index_count = outer_index_remover.unread_outer_index_count_mut();
    write_index_list(
        ctx,
        side,
        cached_outer_indices,
        *outer_index_count,
        list_slot,
    );

    // Increase count to account for values written from cache
    *outer_index_count += cached_count;
}
