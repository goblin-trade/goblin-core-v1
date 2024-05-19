use stylus_sdk::{alloy_primitives::Address, msg};

use crate::{
    program::{try_withdraw, GoblinError, GoblinResult, ReduceOrderError},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw},
    state::{
        BitmapGroup, FIFOMarket, MatchingEngineResponse, OrderId, Side, SlotActions,
        SlotRestingOrder, SlotStorage, TickIndices, TraderState, WritableMarket,
    },
    GoblinMarket,
};

/// Reduce a resting order
/// The size of a resting order is in BaseLots for both bid and quote orders
///
/// # Arguments
///
/// * `side`
/// * `order_id`
/// * `size` - Reduce by this many base lots
/// * `recipient` - Optional. If provided, withdraw freed funds to this address.
///
pub fn process_reduce_order(
    context: &mut GoblinMarket,
    side: Side,
    order_id: &OrderId,
    size: BaseLots,
    recipient: Option<Address>,
) -> GoblinResult<()> {
    let trader = msg::sender();

    // Load states
    let mut slot_storage = SlotStorage::new();
    let market = FIFOMarket::read_from_slot(&slot_storage);
    let mut trader_state = TraderState::read_from_slot(&slot_storage, trader);
    let mut order = SlotRestingOrder::new_from_slot(&slot_storage, order_id);

    let TickIndices {
        outer_index,
        inner_index,
    } = order_id.price_in_ticks.to_indices();

    let mut bitmap_group = BitmapGroup::new_from_slot(&slot_storage, &outer_index);

    let mut mutable_bitmap = bitmap_group.get_bitmap_mut(&inner_index);

    // maintain this pattern
    // find index in the index_list here
    // pass a closure that can remove the index

    // Mutate
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = market
        .reduce_order(
            &mut trader_state,
            &mut order,
            &mut mutable_bitmap,
            trader,
            side,
            order_id,
            size,
            recipient.is_some(),
        )
        .ok_or(GoblinError::ReduceOrderError(ReduceOrderError {}))?;

    // TODO move inside FIFO
    if !bitmap_group.is_active() {
        bitmap_group.inner = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
    }

    // Remove from list
    // problem- the list is dynamic. I want to avoid passing `slot_storage`
    // create a closure to fetch

    // Write states
    trader_state.write_to_slot(&mut slot_storage, trader);
    order.write_to_slot(&mut slot_storage, order_id)?;
    SlotStorage::storage_flush_cache(true);

    // Transfer
    if let Some(recipient) = recipient {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);

        try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;
    }

    Ok(())
}
