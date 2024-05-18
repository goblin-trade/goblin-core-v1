use stylus_sdk::{alloy_primitives::Address, msg};

use crate::{
    parameters::{BASE_LOT_SIZE, QUOTE_LOT_SIZE},
    program::{try_withdraw, GoblinError, GoblinResult, ReduceOrderError},
    quantities::{get_base_atoms_raw, get_quote_atoms_raw, BaseLots},
    state::{
        FIFOMarket, MatchingEngineResponse, OrderId, Side, SlotActions, SlotStorage, WritableMarket,
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

    // Load market
    let mut slot_storage = SlotStorage::new();
    let market = FIFOMarket::read_from_slot(&slot_storage);

    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = market
        .reduce_order(
            &mut slot_storage,
            trader,
            side,
            order_id,
            size,
            recipient.is_some(),
        )
        .ok_or(GoblinError::ReduceOrderError(ReduceOrderError {}))?;

    if let Some(recipient) = recipient {
        let quote_amount = num_quote_lots_out * QUOTE_LOT_SIZE;
        let base_amount = num_base_lots_out * BASE_LOT_SIZE;

        let quote_amount_raw = get_quote_atoms_raw(quote_amount);
        let base_amount_raw = get_base_atoms_raw(base_amount);

        try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;
    }

    Ok(())
}
