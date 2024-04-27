use crate::{
    quantities::{Ticks, WrapperU64},
    state::{Bitmap, BitmapKey, IndexList, OrderId, Side, SlotRestingOrder, SlotStorage},
};

pub struct IterableTickMap {
    pub bid_groups: u16,
    pub ask_groups: u16,
}

impl IterableTickMap {
    /// Insert a resting order at a tick
    /// Used for post orders
    pub fn insert(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        tick: u32,
        resting_order: &SlotRestingOrder,
    ) {
        // Read tick group to see if space is available
        let tick_group_key = BitmapKey::new_from_tick(tick);
        let mut tick_group = Bitmap::new_from_slot(slot_storage, &tick_group_key);

        let bitmap_index = (tick % 32) as usize;
        let mut bitmap = tick_group.orders(bitmap_index);

        match bitmap.best_free_index() {
            None => {
                return;
            }
            Some(index) => {
                // Check whether tick group will become activated. If yes then push to tick_group_list
                let to_activate_group = !tick_group.is_active();

                if to_activate_group {
                    // insert in tick_group_list at correct position
                    let mut tick_group_list: IndexList = IndexList {
                        side: side.clone(),
                        size: self.ask_groups,
                    };
                    tick_group_list.insert(slot_storage, tick_group_key.index);

                    self.increment_group_count(side);

                    // update bitmap
                    bitmap.flip(index);
                    tick_group.update_orders(bitmap_index, &bitmap);
                    tick_group.write_to_slot(slot_storage, &tick_group_key);
                }
                // Save order
                let resting_order_key = OrderId {
                    price_in_ticks: Ticks::new(tick as u64),
                    resting_order_index: index,
                };
                resting_order.write_to_slot(slot_storage, &resting_order_key);
            }
        }
    }

    pub fn increment_group_count(&mut self, side: Side) {
        match side {
            Side::Bid => self.bid_groups += 1,
            Side::Ask => self.ask_groups += 1,
        }
    }
}

#[cfg(test)]
mod test {
    use index_list::{ListKey, ListSlot};
    use stylus_sdk::alloy_primitives::Address;

    use crate::{
        quantities::{BaseLots, WrapperU64},
        state::{index_list, SlotActions},
    };

    use super::*;

    #[test]
    fn test_insert_order() {
        let mut slot_storage = SlotStorage::new();

        let mut pseudo_tree = IterableTickMap {
            bid_groups: 0,
            ask_groups: 0,
        };

        let side = Side::Bid;
        let tick = 20000; // $2k

        let num_base_lots = BaseLots::new(10);

        let resting_order = SlotRestingOrder {
            // trader_address: [0u8; 20],
            trader_address: Address::ZERO,
            num_base_lots,
            last_valid_block: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        };

        pseudo_tree.insert(&mut slot_storage, side.clone(), tick, &resting_order);
        assert_eq!(pseudo_tree.bid_groups, 1);

        let tick_group_key = BitmapKey::new_from_tick(tick);
        assert_eq!(tick_group_key.index, 625);

        let tick_group_0 = Bitmap::new_from_slot(&slot_storage, &tick_group_key);
        assert_eq!(
            tick_group_0.inner,
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );

        let tick_group_item_key = ListKey { index: 0 };
        let tick_group_item = ListSlot::new_from_slot(&slot_storage, &tick_group_item_key);
        assert_eq!(
            tick_group_item.inner,
            [625, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
