use crate::{
    program::{GoblinResult, OrderToInsert},
    state::{slot_storage, MarketState, Side, SlotStorage},
};

use super::SlotRestingOrder;

pub struct InsertableBook {}

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

    pub fn insert_in_index_list(
        &mut self,
        side: Side,
        market_state: &mut MarketState,
        order: OrderToInsert,
        index: usize,
    ) {
    }
}
