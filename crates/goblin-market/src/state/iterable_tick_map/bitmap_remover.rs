use crate::state::{OrderId, OuterIndex, SlotStorage, TickIndices};

use super::BitmapGroup;

/// Facilitates efficient batch deactivations in bitmap groups
pub struct BitmapRemover {
    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,
}

impl BitmapRemover {
    pub fn new() -> Self {
        BitmapRemover {
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
        }
    }

    /// Write cached bitmap group to slot
    /// This should be called when the outer index changes during looping,
    /// and when the loop is complete
    pub fn write_last_bitmap_group(&self, slot_storage: &mut SlotStorage) {
        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
        }
    }

    /// Turn off a bit at a given (outer index, inner index, resting order index)
    /// If the outer index changes, then the previous bitmap is overwritten
    ///
    /// write_last_bitmap_group() must be called after deactivations are complete to write
    /// the last bitmap group to slot.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `order_id`
    ///
    pub fn deactivate(&mut self, slot_storage: &mut SlotStorage, order_id: &OrderId) {
        let TickIndices {
            outer_index,
            inner_index,
        } = order_id.price_in_ticks.to_indices();

        // If last outer index has not changed, re-use the cached bitmap group.
        // Else load anew and update the cache.
        if self.last_outer_index != Some(outer_index) {
            // Outer index changed. Flush the old bitmap group to slot.
            self.write_last_bitmap_group(slot_storage);

            self.bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            self.last_outer_index = Some(outer_index);
        }

        let mut bitmap = self.bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.clear(&order_id.resting_order_index);
    }
}
