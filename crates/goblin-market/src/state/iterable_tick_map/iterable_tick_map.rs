use crate::{
    quantities::Ticks,
    state::{BitmapGroup, IndexList, OrderId, Side, SlotRestingOrder, SlotStorage},
};

use super::TickIndices;

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
        price_in_ticks: Ticks,
        resting_order: &SlotRestingOrder,
    ) {
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, &outer_index);

        let mut bitmap = bitmap_group.bitmap_at(inner_index.clone());

        match bitmap.best_free_index() {
            None => {
                return;
            }
            Some(resting_order_index) => {
                // Check whether tick group will become activated. If yes then push to tick_group_list
                let to_activate_group = !bitmap_group.is_active();

                if to_activate_group {
                    // insert in tick_group_list at correct position
                    let mut index_list = IndexList {
                        side: side.clone(),
                        size: self.ask_groups,
                    };
                    index_list.insert(slot_storage, outer_index.as_u16());

                    self.increment_group_count(side);

                    // update bitmap
                    bitmap.flip(resting_order_index.clone());
                    bitmap_group.update_bitmap(inner_index, &bitmap);
                    bitmap_group.write_to_slot(slot_storage, &outer_index);
                }
                // Save order
                let resting_order_key = OrderId {
                    price_in_ticks,
                    resting_order_index,
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
        let price_in_ticks = Ticks::new(20000); // $2k

        let num_base_lots = BaseLots::new(10);

        let resting_order = SlotRestingOrder {
            trader_address: Address::ZERO,
            num_base_lots,
            last_valid_block: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        };

        pseudo_tree.insert(
            &mut slot_storage,
            side.clone(),
            price_in_ticks,
            &resting_order,
        );
        assert_eq!(pseudo_tree.bid_groups, 1);

        let TickIndices {
            outer_index,
            inner_index: _,
        } = price_in_ticks.to_indices();

        assert_eq!(outer_index.as_u16(), 625);

        let tick_group_0 = BitmapGroup::new_from_slot(&slot_storage, &outer_index);
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
