use crate::state::{
    RestingOrderKey, Side, SlotRestingOrder, SlotStorage, TickGroup, TickGroupKey, TickGroupList,
};

pub struct PseudoTree {
    pub market_index: u8,
    pub bid_groups: u16,
    pub ask_groups: u16,
}

impl PseudoTree {
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
        let tick_group_key = TickGroupKey::new_from_tick(self.market_index, tick);
        let mut tick_group = TickGroup::new_from_slot(slot_storage, &tick_group_key);

        let bitmap_index = (tick % 32) as usize;
        let mut bitmap = tick_group.bitmap(bitmap_index);

        match bitmap.best_free_index() {
            None => {
                return;
            }
            Some(index) => {
                // Check whether tick group will become activated. If yes then push to tick_group_list
                let to_activate_group = !tick_group.is_active();

                if to_activate_group {
                    // insert in tick_group_list at correct position
                    let mut tick_group_list = TickGroupList {
                        market_index: self.market_index,
                        side: side.clone(),
                        size: self.ask_groups,
                    };
                    tick_group_list.insert(slot_storage, tick_group_key.index);

                    self.increment_group_count(side);

                    // update bitmap
                    bitmap.flip(index);
                    tick_group.update_bitmap(bitmap_index, &bitmap);
                    tick_group.write_to_slot(slot_storage, &tick_group_key);
                }
                // Save order
                let resting_order_key = RestingOrderKey {
                    market_index: self.market_index,
                    tick,
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
    use tick_group_list::{TickGroupItem, TickGroupItemKey};

    use crate::state::{tick_group_list, SlotActions, SlotKey};

    use super::*;

    #[test]
    fn test_insert_order() {
        let mut slot_storage = SlotStorage::new();

        let market_index = 0;

        let mut pseudo_tree = PseudoTree {
            market_index,
            bid_groups: 0,
            ask_groups: 0,
        };

        let side = Side::Bid;
        let tick = 20000; // $2k

        let resting_order = SlotRestingOrder {
            trader_address: [0u8; 20],
            num_base_lots: 10,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        };

        pseudo_tree.insert(&mut slot_storage, side.clone(), tick, &resting_order);
        assert_eq!(pseudo_tree.bid_groups, 1);

        let tick_group_key = TickGroupKey::new_from_tick(market_index, tick);
        assert_eq!(tick_group_key.index, 625);

        let tick_group_0 = TickGroup::new_from_slot(&slot_storage, &tick_group_key);
        assert_eq!(
            tick_group_0.inner,
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );

        let tick_group_item_key = TickGroupItemKey {
            market_index,
            index: 0,
        };
        let tick_group_item = TickGroupItem::new_from_slot(&slot_storage, &tick_group_item_key);
        assert_eq!(
            tick_group_item.inner,
            [625, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
