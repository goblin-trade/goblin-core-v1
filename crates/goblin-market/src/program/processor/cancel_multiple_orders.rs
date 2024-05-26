use alloc::vec::Vec;
use stylus_sdk::{
    alloy_primitives::{Address, B256},
    msg,
};

use crate::{
    program::GoblinResult,
    state::{FIFOMarket, OrderId, SlotActions, SlotStorage, TraderState},
    GoblinMarket,
};

/// TODO hack- we can have a single reduce_multiple_orders() function
/// This can be used to reduce or cancel multiple orders at once
pub fn process_multiple_orders_by_id(
    context: &mut GoblinMarket,
    order_ids: Vec<B256>,
    recipient: Option<Address>,
) -> GoblinResult<()> {
    let trader = msg::sender();

    // Load states
    let mut slot_storage = SlotStorage::new();
    let mut market = FIFOMarket::read_from_slot(&slot_storage);
    let mut trader_state = TraderState::read_from_slot(&slot_storage, trader);

    // When cancelling multiple orders
    // - market, trader state keeps getting updated in memory
    // - multiple RestingOrders are modified
    // - Variable number of `index_list` elements are updated

    // `index_list` updation should be optimized
    // - keep caching and removing items until IDs to remove are exhausted
    // - In reduce_order, read and update bitmap group only for orders that will be closed
    // - Orders with the same outer_index share BitmapGroups. Do not double read BitmapGroups

    // Alternative- modify index_list and market at the end, after seeing which orders were closed
    // This keeps the code clean

    Ok(())
}
