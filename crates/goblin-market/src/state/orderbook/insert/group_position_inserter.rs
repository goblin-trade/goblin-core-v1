use crate::state::{
    bitmap_group::BitmapGroup, order::group_position::GroupPosition, ArbContext, MarketPrices,
    OuterIndex,
};

/// Facilitates efficient batch activations in bitmap groups
pub struct GroupPositionInserter {
    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when a new outer index is encountered.
    pub bitmap_group: BitmapGroup,

    /// Outer index corresponding to `bitmap_group`
    pub last_outer_index: Option<OuterIndex>,
}

impl GroupPositionInserter {
    pub fn new() -> Self {
        GroupPositionInserter {
            bitmap_group: BitmapGroup::default(),
            last_outer_index: None,
        }
    }

    /// Activate an order in the current bitmap group at the given GroupPosition
    ///
    /// Externally ensure that load_outer_index() was called first so that
    /// `last_outer_index` is not None
    ///
    pub fn activate_in_current(&mut self, group_position: GroupPosition) {
        let mut bitmap = self
            .bitmap_group
            .get_bitmap_mut(&group_position.inner_index);
        bitmap.activate(&group_position.resting_order_index);
    }

    /// Loads a new bitmap group for the new outer index, flushing the previous group.
    /// No-op if outer index does not change
    ///
    /// Externally ensure that we always move away from the centre
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `outer_index` - Load bitmap group for this index
    /// * `bitmap_group_is_empty` - Avoid SLOAD if the bitmap group is known to be empty
    ///
    pub fn load_outer_index(
        &mut self,
        slot_storage: &mut ArbContext,
        outer_index: OuterIndex,
        bitmap_group_is_empty: bool,
    ) {
        if self.last_outer_index == Some(outer_index) {
            return;
        }
        // Outer index changed. Flush the old bitmap group to slot.
        self.flush_bitmap_group(slot_storage);

        // Update outer index and load new bitmap group from slot
        self.last_outer_index = Some(outer_index);

        self.bitmap_group = if bitmap_group_is_empty {
            // Gas optimization- avoid SLOAD if the group was inactive before
            BitmapGroup::default()
        } else {
            BitmapGroup::new_from_slot(slot_storage, outer_index)
        };
    }

    /// Clear garbage bits in the bitmap group that fall between best market prices
    ///
    /// # Arguments
    ///
    /// * `best_market_prices`
    ///
    pub fn clear_garbage_bits(&mut self, best_market_prices: &MarketPrices) {
        if let Some(outer_index) = self.last_outer_index {
            self.bitmap_group
                .clear_garbage_bits(outer_index, best_market_prices);
        }
    }

    /// Write cached bitmap group to slot
    /// This should be called when the outer index changes during looping,
    /// and when the loop is complete
    pub fn flush_bitmap_group(&self, slot_storage: &mut ArbContext) {
        if let Some(last_index) = self.last_outer_index {
            self.bitmap_group.write_to_slot(slot_storage, &last_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        quantities::{Ticks, WrapperU64},
        state::{order::order_id::OrderId, ContextActions, RestingOrderIndex},
    };

    /// Activates an order ID. If the outer index changes, the previous bitmap group is flushed
    ///
    /// Identical to `RestingOrderInserter::activate_order_id()`. It is replicated here
    /// for isolated testing.
    ///
    fn activate_order_id(
        bitmap_inserter: &mut GroupPositionInserter,
        slot_storage: &mut ArbContext,
        order_id: &OrderId,
        outer_index_inserted: bool,
    ) {
        let outer_index = order_id.price_in_ticks.outer_index();
        bitmap_inserter.load_outer_index(slot_storage, outer_index, outer_index_inserted);
        bitmap_inserter.activate_in_current(GroupPosition::from(order_id));
    }

    #[test]
    fn insert_single_order() {
        let slot_storage = &mut ArbContext::new();
        let bitmap_group_is_empty = true;

        let mut inserter = GroupPositionInserter::new();

        let order_id = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id.price_in_ticks.outer_index();

        // 1. Activate and check
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id,
            bitmap_group_is_empty,
        );
        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.flush_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_same_tick() {
        let slot_storage = &mut ArbContext::new();
        let bitmap_group_is_empty = true;

        let mut inserter = GroupPositionInserter::new();

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
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_0,
            bitmap_group_is_empty,
        );
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_1,
            bitmap_group_is_empty,
        );

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000011;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.flush_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_different_inner_indices_on_same_group() {
        let slot_storage = &mut ArbContext::new();
        let bitmap_group_is_empty = true;

        let mut inserter = GroupPositionInserter::new();

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
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_0,
            bitmap_group_is_empty,
        );
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_1,
            bitmap_group_is_empty,
        );

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[1] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.flush_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_in_different_groups() {
        let slot_storage = &mut ArbContext::new();
        let bitmap_group_is_empty = true;

        let mut inserter = GroupPositionInserter::new();

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
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_0,
            bitmap_group_is_empty,
        );
        activate_order_id(
            &mut inserter,
            slot_storage,
            &order_id_1,
            bitmap_group_is_empty,
        ); // this will write first group

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
        inserter.flush_bitmap_group(slot_storage);
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);
    }
}
