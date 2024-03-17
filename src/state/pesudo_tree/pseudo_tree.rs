use crate::state::{
    CBRestingOrder, RestingOrderKey, Side, SlotKey, SlotStorage, TickGroup, TickGroupKey,
    TickGroupList, MAX_ORDERS_PER_TICK,
};

use super::{tick_group, tick_group_list};

pub struct PseudoTree {
    pub market_index: u8,
    pub tick_groups_count_for_bids: u16,
    pub tick_groups_count_for_asks: u16,
}

impl PseudoTree {
    /// Insert a resting order at a tick
    /// Used for post orders
    fn insert(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        tick: u32,
        resting_order: &CBRestingOrder,
    ) {
        // Read tick group to see if space is available
        let tick_group_key = TickGroupKey::new_from_tick(self.market_index, tick);
        let tick_group = TickGroup::new_from_slot(slot_storage, &tick_group_key);

        let header_index = (tick % 32) as usize;
        let header = tick_group.bitmap(header_index);

        match header.best_free_index() {
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
                        side,
                        size: self.tick_groups_count_for_asks,
                    };

                    // increase size- tick_groups_count_for_asks

                    tick_group_list.insert(slot_storage, tick_group_key.index);
                }
                // Save order
                // update bitmap
            }
        }
        // if header.order_count == MAX_ORDERS_PER_TICK {
        //     return;
        // }

        // Why not store bitmap of active ticks instead of order_count and header?
        // This allows us to fit orders in the best slot, removing the need to loop.
        // We don't need a circular buffer as well.
        // A 1 byte bitmap can track 8 resting orders, which is enough. No need to change
        // tick_group_list
    }
}
