use crate::state::{InnerIndex, OrderId, OuterIndex, RestingOrderIndex, SlotStorage, TickIndices};

use super::BitmapGroup;

/// Facilitates efficient batch deactivations in bitmap groups
pub struct BitmapRemover {
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
    pub fn new() -> Self {
        BitmapRemover {
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
            pending_write: false,
        }
    }

    /// Write cached bitmap group to slot
    /// This should be called when the outer index changes during looping,
    /// and when the loop is complete
    pub fn write_last_bitmap_group(&mut self, slot_storage: &mut SlotStorage) {
        if !self.pending_write {
            return;
        }
        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
            self.pending_write = false;
        }
    }

    /// Loads a new bitmap group for the new outer index. The previous group is flushed.
    /// No-op if outer index does not change
    ///
    /// Externally ensure that we always move away from the centre
    ///
    pub fn set_outer_index(&mut self, slot_storage: &mut SlotStorage, outer_index: OuterIndex) {
        if self.last_outer_index != Some(outer_index) {
            // Outer index changed. Flush the old bitmap group to slot.
            self.write_last_bitmap_group(slot_storage);
            self.bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
            self.last_outer_index = Some(outer_index);
        }
    }

    /// Whether a resting order is present at given (inner_index, resting_order_index)
    pub fn order_present(
        &self,
        inner_index: InnerIndex,
        resting_order_index: RestingOrderIndex,
    ) -> bool {
        assert!(
            self.last_outer_index.is_some(),
            "Outer index is None, no bitmap group loaded"
        );

        let bitmap = self.bitmap_group.get_bitmap(&inner_index);

        bitmap.order_present(resting_order_index)
    }

    /// Turn off a bit at a given (outer index, inner index, resting order index)
    /// If the outer index changes, then the previous bitmap is overwritten
    ///
    /// write_last_bitmap_group() must be called after deactivations are complete to write
    /// the last bitmap group to slot.
    ///
    /// Externally ensure that the bit at `order_id` is active, else `pending_write` is
    /// set to true leading to a wasted slot write.
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
        self.set_outer_index(slot_storage, outer_index);

        let mut bitmap = self.bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.clear(&order_id.resting_order_index);
        self.pending_write = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::{Ticks, WrapperU64},
        state::{BitmapGroup, OrderId, RestingOrderIndex, SlotActions, SlotStorage},
    };

    use super::BitmapRemover;

    #[test]
    fn deactivate_on_blank_bitmap_group() {
        let slot_storage = &mut SlotStorage::new();
        let mut remover = BitmapRemover::new();

        let order_id = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id.price_in_ticks.outer_index();

        // 1. Deactivate and check
        remover.deactivate(slot_storage, &order_id);
        assert_eq!(outer_index, remover.last_outer_index.unwrap());
        assert!(remover.pending_write);

        // Expected bitmap group is still blank since it was already off
        let expected_bitmap_group = BitmapGroup::default();
        assert_eq!(expected_bitmap_group, remover.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        // Since pending_write is true, this will lead to a wasted slot write
        remover.write_last_bitmap_group(slot_storage);
        assert!(!remover.pending_write);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn deactivate_single_order() {
        let slot_storage = &mut SlotStorage::new();

        let mut remover = BitmapRemover::new();

        let order_id = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };

        // First activate the order to set it up
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.write_to_slot(slot_storage, &order_id.price_in_ticks.outer_index());

        // 1. Deactivate and check
        remover.deactivate(slot_storage, &order_id);
        assert!(remover.pending_write);
        assert_eq!(
            order_id.price_in_ticks.outer_index(),
            remover.last_outer_index.unwrap()
        );

        let expected_bitmap_group = BitmapGroup::default(); // Bit should now be cleared
        assert_eq!(expected_bitmap_group, remover.bitmap_group);

        let mut read_bitmap_group =
            BitmapGroup::new_from_slot(slot_storage, order_id.price_in_ticks.outer_index());
        assert_eq!(bitmap_group, read_bitmap_group);

        // 2. Write to slot and check
        remover.write_last_bitmap_group(slot_storage);
        assert!(!remover.pending_write);
        read_bitmap_group =
            BitmapGroup::new_from_slot(slot_storage, order_id.price_in_ticks.outer_index());
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn deactivate_two_orders_on_same_tick() {
        let slot_storage = &mut SlotStorage::new();

        let mut remover = BitmapRemover::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(1),
        };

        // First activate the orders to set them up
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000011;
        bitmap_group.write_to_slot(slot_storage, &order_id_0.price_in_ticks.outer_index());

        // 1. Deactivate both and check
        remover.deactivate(slot_storage, &order_id_0);
        remover.deactivate(slot_storage, &order_id_1);

        assert!(remover.pending_write);

        let outer_index = order_id_0.price_in_ticks.outer_index();
        assert_eq!(outer_index, remover.last_outer_index.unwrap());

        let expected_bitmap_group = BitmapGroup::default(); // Both bits should now be cleared
        assert_eq!(expected_bitmap_group, remover.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(bitmap_group, read_bitmap_group);

        // 2. Write to slot and check
        remover.write_last_bitmap_group(slot_storage);
        assert!(!remover.pending_write);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn deactivate_two_orders_on_different_inner_indices_on_same_bitmap_group() {
        let slot_storage = &mut SlotStorage::new();

        let mut remover = BitmapRemover::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ONE,
            resting_order_index: RestingOrderIndex::new(0),
        };

        // First activate the orders to set them up
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b00000001;
        bitmap_group.inner[1] = 0b00000001;
        bitmap_group.write_to_slot(slot_storage, &order_id_0.price_in_ticks.outer_index());

        // 1. Deactivate both and check
        remover.deactivate(slot_storage, &order_id_0);
        remover.deactivate(slot_storage, &order_id_1);

        let outer_index = order_id_0.price_in_ticks.outer_index();
        assert_eq!(outer_index, remover.last_outer_index.unwrap());

        let expected_bitmap_group = BitmapGroup::default(); // Both bits should now be cleared
        assert_eq!(expected_bitmap_group, remover.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(bitmap_group, read_bitmap_group);

        // 2. Write to slot and check
        remover.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn deactivate_two_orders_on_different_bitmap_groups() {
        let slot_storage = &mut SlotStorage::new();

        let mut remover = BitmapRemover::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::new(32),
            resting_order_index: RestingOrderIndex::new(0),
        };

        // First activate the orders to set them up
        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[0] = 0b00000001;
        bitmap_group_0.write_to_slot(slot_storage, &order_id_0.price_in_ticks.outer_index());

        let mut bitmap_group_1 = BitmapGroup::default();
        bitmap_group_1.inner[0] = 0b00000001;
        bitmap_group_1.write_to_slot(slot_storage, &order_id_1.price_in_ticks.outer_index());

        // 1. Deactivate both and check
        remover.deactivate(slot_storage, &order_id_0);
        remover.deactivate(slot_storage, &order_id_1); // this will write the first bitmap group

        let outer_index_0 = order_id_0.price_in_ticks.outer_index();
        let outer_index_1 = order_id_1.price_in_ticks.outer_index();

        let expected_bitmap_group_0 = BitmapGroup::default();
        let expected_bitmap_group_1 = BitmapGroup::default();

        // bitmap_group_0 has been written to slot
        let read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(expected_bitmap_group_0, read_bitmap_group_0);

        // bitmap_group_1 and outer_index_1 are still in cache
        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(bitmap_group_1, read_bitmap_group_1); // Still has the bit on in the cache
        assert_eq!(outer_index_1, remover.last_outer_index.unwrap());
        assert_eq!(expected_bitmap_group_1, remover.bitmap_group);

        // 2. Write cache
        remover.write_last_bitmap_group(slot_storage);
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);
    }
}
