use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup, order::group_position::GroupPosition, InnerIndex, OuterIndex,
        Side, SlotStorage,
    },
};

/// Facilitates efficient batch deactivations in bitmap groups
pub struct BitmapRemover {
    /// Whether for bids or asks
    /// Traverse upwards (ascending) for asks and downwards (descending) for bids
    pub side: Side,

    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,

    /// Whether the bitmap group was updated in memory and is pending a write.
    /// write_last_bitmap_group() should write to slot only if this is true.
    pub pending_write: bool,
}

impl BitmapRemover {
    pub fn new(side: Side) -> Self {
        BitmapRemover {
            side,
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
            pending_write: false,
        }
    }

    /// Whether a resting order is present at given (inner_index, resting_order_index)
    ///
    /// Externally ensure that set_outer_index() was called first so that
    /// `last_outer_index` is not None
    ///
    pub fn order_present(&self, group_position: GroupPosition) -> bool {
        let bitmap = self.bitmap_group.get_bitmap(&group_position.inner_index);
        bitmap.order_present(group_position.resting_order_index)
    }

    /// Deactivate an order in the current bitmap group
    ///
    /// Externally ensure that set_outer_index() was called first so that
    /// `last_outer_index` is not None
    ///
    pub fn deactivate_in_current(&mut self, group_position: GroupPosition) {
        let mut bitmap = self
            .bitmap_group
            .get_bitmap_mut(&group_position.inner_index);
        bitmap.clear(&group_position.resting_order_index);
        self.pending_write = true;
    }

    // Get price of the best active order in the current bitmap group,
    // beginning from a given position
    //
    // # Arguments
    //
    /// * `starting_index` - Search beginning from this index (inclusive) if Some,
    /// else begin lookup from the edge of the bitmap group.
    ///
    pub fn get_best_price_in_current(&self, starting_index: Option<InnerIndex>) -> Option<Ticks> {
        if let (Some(outer_index), Some(inner_index)) = (
            self.last_outer_index,
            self.bitmap_group
                .best_active_inner_index(self.side, starting_index),
        ) {
            Some(Ticks::from_indices(outer_index, inner_index))
        } else {
            None
        }
    }

    /// Loads a new bitmap group for the new outer index. The previous group is flushed.
    /// No-op if outer index does not change
    ///
    /// Externally ensure that we always move away from the centre
    ///
    pub fn load_outer_index(&mut self, slot_storage: &mut SlotStorage, outer_index: OuterIndex) {
        if self.last_outer_index == Some(outer_index) {
            return;
        }
        // Outer index changed. Flush the old bitmap group to slot.
        self.flush_bitmap_group(slot_storage);

        // Update outer index and load new bitmap group from slot
        self.last_outer_index = Some(outer_index);
        self.bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
    }

    /// Flush the cached bitmap group to slot
    /// This should be called before moving to a new outer index
    pub fn flush_bitmap_group(&mut self, slot_storage: &mut SlotStorage) {
        if !self.pending_write {
            return;
        }
        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
            self.pending_write = false;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         quantities::{Ticks, WrapperU64},
//         state::{BitmapGroup, OrderId, RestingOrderIndex, Side, SlotActions, SlotStorage},
//     };

//     use super::*;
// }
