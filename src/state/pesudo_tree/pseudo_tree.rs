use crate::state::{CBRestingOrder, RestingOrderKey, Side, SlotKey, SlotStorage, TickGroup, TickGroupKey, MAX_ORDERS_PER_TICK};

use super::tick_group;

pub struct PseudoTree {
    pub market_index: u8,
    pub tick_groups_count_for_bids: u32,
    pub tick_groups_count_for_asks: u32,
}

impl PseudoTree {
    /// Insert a resting order at a tick
    /// Used for post orders
    fn insert(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        tick: u32,
        resting_order: &CBRestingOrder
    ) {
        // Read tick group to see if space is available
        let tick_group_key = TickGroupKey::new_from_tick(self.market_index, tick);
        let tick_group = TickGroup::new_from_slot(slot_storage, &tick_group_key);

        let header_index = (tick % tick_group_key.index as u32) as u8;
        let header = tick_group.header(header_index);

        if header.order_count == MAX_ORDERS_PER_TICK {
            return;
        }

        // Why not store bitmap of active ticks instead of order_count and header?
        // This allows us to fit orders in the best slot, removing the need to loop.
        // We don't need a circular buffer as well.
        // Disadvantage- need 16 bits for 16 orders. This means max tick is reduced to 2^20
        // This also increases the number of elements in tick_group_list. 1 tick group is worth $1.6
        // difference instead of $3.2
        // Alternative- max 8 orders per tick. Is it too less?
    }
}