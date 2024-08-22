use crate::{
    program::{GoblinResult, OrderToInsert},
    state::{slot_storage, MarketState, Side, SlotStorage},
};

use alloc::vec::Vec;

use super::{ListKey, ListSlot, OuterIndex, SlotRestingOrder};

pub struct InsertableBook {
    outer_index_stack: Vec<OuterIndex>,

    /// Slot index of the list item pending a read
    slot_index: u16,

    /// Relative index of the list item pending a read
    relative_index: u16,

    /// List slot for `slot_index`
    list_slot: ListSlot,

    /// Whether the list is exhausted. Set to true when the last
    /// element at (0, 0) is pushed to the stack
    list_exhausted: bool,
}

impl InsertableBook {
    // This function can be moved out of the struct. State variables are not needed.
    pub fn insert_resting_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        market_state: &mut MarketState,
        order: OrderToInsert,
        index: usize,
    ) -> GoblinResult<()> {
        let OrderToInsert {
            order_id,
            resting_order,
        } = order;

        // 1. Update market state
        // Optimization- since the first element is closest to the centre, we only need
        // to check the first element against the current best price.
        // Update the best price if necessary.
        if index == 0 {
            // Update best market price
            if side == Side::Bid && order_id.price_in_ticks > market_state.best_bid_price {
                market_state.best_bid_price = order_id.price_in_ticks;
            }

            if side == Side::Ask && order_id.price_in_ticks < market_state.best_ask_price {
                market_state.best_ask_price = order_id.price_in_ticks;
            }
        }

        // 2. Write resting order to slot
        resting_order.write_to_slot(slot_storage, &order_id)?;

        // 3. Try to insert outer index in list
        // Find whether it was inserted or whether it was already present
        //

        Ok(())
    }

    pub fn new(slot_storage: &SlotStorage, outer_index_count: u16) -> Self {
        let slot_index = (outer_index_count - 1) / 16;

        InsertableBook {
            outer_index_stack: Vec::new(),
            slot_index,
            relative_index: (outer_index_count - 1) % 16,
            list_slot: ListSlot::new_from_slot(slot_storage, ListKey { index: slot_index }),
            list_exhausted: outer_index_count == 0,
        }
    }

    /// Prepare an outer index to be written in the index list
    ///
    /// This function loops through the index list till the valid position is found.
    /// Read values and the current value to be written is pushed in a stash.
    ///
    /// Returns true if the value was queued and false if not (when value is already present)
    ///
    pub fn push_outer_index(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        market_state: &MarketState,
        outer_index: OuterIndex,
        index: usize,
    ) -> bool {
        // Outer index is already in list.
        // The incoming outer index cannot be worse than the last pushed index
        // so we can skip those checks and only check for equality.
        if self
            .outer_index_stack
            .last()
            .is_some_and(|last_pushed_index| *last_pushed_index == outer_index)
        {
            return false;
        }

        if self.list_exhausted {
            self.outer_index_stack.push(outer_index);
            return true;
        }

        // TODO need looping logic
        // keep reading from index list till correct position is found

        // 1. Loop through index slots to generate outer_index_stack and find indices
        'list_loop: loop {
            let list_key = ListKey {
                index: self.slot_index,
            };
            self.list_slot = ListSlot::new_from_slot(slot_storage, list_key);

            break;
        }

        // Pop next item from index list
        let current_outer_index = self.list_slot.get(self.relative_index as usize);

        if outer_index == current_outer_index {
            return false;
        } else if (side == Side::Bid && outer_index > current_outer_index)
            || (side == Side::Ask && outer_index < current_outer_index)
        {
        }
        true
    }
}
