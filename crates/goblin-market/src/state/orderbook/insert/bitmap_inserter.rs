use crate::state::{
    bitmap_group::BitmapGroup, order::order_id::OrderId, OuterIndex, SlotStorage, TickIndices,
};

/// Facilitates efficient batch activations in bitmap groups
pub struct BitmapInserter {
    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,
}

impl BitmapInserter {
    pub fn new() -> Self {
        BitmapInserter {
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

    /// Turn on a bit at a given (outer index, inner index, resting order index)
    /// If the outer index changes then the previous bitmap is overwritten
    ///
    /// write_last_bitmap_group() must be called after activations are complete to write
    /// the last bitmap group to slot.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `order_id`
    /// * `new_group` - Whether the group is empty. If true we can start with a blank
    /// bitmap group instead of wasting gas on SLOAD.
    ///
    pub fn activate(
        &mut self,
        slot_storage: &mut SlotStorage,
        order_id: &OrderId,
        bitmap_group_is_empty: bool,
    ) {
        let TickIndices {
            outer_index,
            inner_index,
        } = order_id.price_in_ticks.to_indices();

        // If last outer index has not changed, re-use the cached bitmap group.
        // Else load anew and update the cache.
        if self.last_outer_index != Some(outer_index) {
            // Outer index changed. Flush the old bitmap group to slot.
            self.write_last_bitmap_group(slot_storage);

            self.bitmap_group = if bitmap_group_is_empty {
                BitmapGroup::default()
            } else {
                BitmapGroup::new_from_slot(slot_storage, outer_index)
            };

            self.last_outer_index = Some(outer_index);
        }

        let mut bitmap = self.bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.activate(&order_id.resting_order_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        quantities::{Ticks, WrapperU64},
        state::{RestingOrderIndex, SlotActions},
    };

    #[test]
    fn insert_single_order() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id.price_in_ticks.outer_index();

        // 1. Activate and check
        inserter.activate(slot_storage, &order_id, bitmap_group_is_empty);
        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_same_tick() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(1),
        };

        let outer_index = order_id_0.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty);

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000011;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_different_inner_indices_on_same_group() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ONE,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id_0.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty);

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[1] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_in_different_groups() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::new(32),
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index_0 = order_id_0.price_in_ticks.outer_index();
        let outer_index_1 = order_id_1.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty); // this will write first group

        let mut expected_bitmap_group_0 = BitmapGroup::default();
        expected_bitmap_group_0.inner[0] = 0b00000001;
        let expected_bitmap_group_1 = expected_bitmap_group_0;

        // bitmap_group_0 has been written to slot
        let read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(expected_bitmap_group_0, read_bitmap_group_0);

        // bitmap_group_1 and outer_index_1 are still in cache
        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(BitmapGroup::default(), read_bitmap_group_1);
        assert_eq!(outer_index_1, inserter.last_outer_index.unwrap());
        assert_eq!(expected_bitmap_group_1, inserter.bitmap_group);

        // 2. Write cache
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);
    }
}
