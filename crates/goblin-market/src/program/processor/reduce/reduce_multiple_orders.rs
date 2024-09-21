use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, FixedBytes};

use crate::{
    program::{
        try_withdraw, types::matching_engine_response::MatchingEngineResponse, FailedToReduce,
        GoblinError, GoblinResult,
    },
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, Ticks, WrapperU64},
    state::{
        order::{
            order_id::OrderId,
            resting_order::{ReduceOrderInnerResponse, RestingOrder, SlotRestingOrder},
        },
        MarketState, RestingOrderIndex, SlotActions, SlotStorage, TraderState,
    },
    GoblinMarket,
};

use super::RemoveMultipleManager;

pub struct ReduceOrderPacket {
    // ID of order to reduce
    pub order_id: OrderId,

    // Reduce at most these many lots. Pass u64::MAX to close
    pub lots_to_remove: BaseLots,

    // Revert entire TX if reduction fails for this order
    pub revert_if_fail: bool,
}

impl From<&FixedBytes<17>> for ReduceOrderPacket {
    fn from(bytes: &FixedBytes<17>) -> Self {
        ReduceOrderPacket {
            order_id: OrderId {
                price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
                resting_order_index: RestingOrderIndex::new(bytes[8]),
            },
            lots_to_remove: BaseLots::new(u64::from_be_bytes(bytes[9..16].try_into().unwrap())),
            revert_if_fail: (bytes[16] & 0b0000_0001) != 0,
        }
    }
}

/// Try to reduce or cancel one or more resting orders. Pass MAX amount to cancel an order.
///
/// # Arguments
///
/// * `context`
/// * `trader` - Reduce orders belonging to this trader
/// * `order_packets`
/// * `claim_funds` - Whether to claim ERC20 tokens to wallet, or whether to credit
/// them to trader state
///
pub fn process_reduce_multiple_orders(
    context: &mut GoblinMarket,
    trader: Address,
    order_packets: Vec<FixedBytes<17>>,
    claim_funds: bool,
) -> GoblinResult<()> {
    // Read
    let slot_storage = &mut SlotStorage::new();
    let market_state = &mut MarketState::read_from_slot(slot_storage);
    let trader_state = &mut TraderState::read_from_slot(slot_storage, trader);

    // Mutate
    // State reads and writes are performed inside reduce_multiple_orders_inner()
    // The number of slot reads is dynamic
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = reduce_multiple_orders_inner(
        slot_storage,
        market_state,
        trader,
        trader_state,
        order_packets,
        claim_funds,
    )?;

    // Write state
    market_state.write_to_slot(slot_storage)?;
    trader_state.write_to_slot(slot_storage, trader);
    SlotStorage::storage_flush_cache(true);

    // Transfer tokens
    if claim_funds {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);
        try_withdraw(context, quote_amount_raw, base_amount_raw, trader)?;
    }

    Ok(())
}

/// Try to reduce multiple orders by ID
///
/// It is possible that an order ID is already closed, and also occupied by
/// another trader. The current behavior is that if one reduction fails,
/// continue trying to reduction others.
///
/// Order IDs should be in correct order
/// - Asks in ascending order and bids in desecnding order of price. The call reverts if
/// they are not in order.
/// - Order IDs for bids and asks can be interspersed but the sorting in the
/// respective side must be maintained.
///
/// Reduction involves
///
/// - Updating trader state
/// - Updating / closing the order slot
/// - Updating the bitmap
/// - Removing the outer index from index list if the outer index is closed
/// - Updating outer index sizes and best prices in market state
///
/// Opportunity to use VM cache is limited to bitmap group. We need order IDs in
/// correct order for index list updations
///
/// # Arguments
///
/// * `slot_storage`
/// * `market_state`
/// * `trader`
/// * `trader_state`
/// * `order_packets`
/// * `claim_funds`
///
pub fn reduce_multiple_orders_inner(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    trader: Address,
    trader_state: &mut TraderState,
    order_packets: Vec<FixedBytes<17>>,
    claim_funds: bool,
) -> GoblinResult<MatchingEngineResponse> {
    let mut quote_lots_released = QuoteLots::ZERO;
    let mut base_lots_released = BaseLots::ZERO;

    let mut manager = RemoveMultipleManager::new(
        market_state.bids_outer_indices,
        market_state.asks_outer_indices,
    );

    for order_packet_bytes in order_packets {
        let ReduceOrderPacket {
            order_id,
            lots_to_remove,
            revert_if_fail,
        } = ReduceOrderPacket::from(&order_packet_bytes);

        let side = order_id.side(market_state);
        let order_present = manager.find_order(slot_storage, side, order_id)?;

        if !order_present {
            if revert_if_fail {
                return Err(GoblinError::FailedToReduce(FailedToReduce {}));
            }
            continue;
        }

        let mut resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

        if trader != resting_order.trader_address {
            return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        }

        let matching_engine_response = resting_order.reduce_order_v2(
            trader_state,
            &order_id,
            side,
            lots_to_remove,
            false,
            claim_funds,
        );

        quote_lots_released += matching_engine_response.num_quote_lots_out;
        base_lots_released += matching_engine_response.num_base_lots_out;

        if resting_order.is_empty() {
            manager.remove_order(slot_storage, market_state);
        } else {
            resting_order.write_to_slot(slot_storage, &order_id)?;
        }

        // if order_present {
        //     let mut resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

        //     if trader != resting_order.trader_address {
        //         return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        //     }

        //     let matching_engine_response = resting_order.reduce_order_v2(
        //         trader_state,
        //         &order_id,
        //         side,
        //         lots_to_remove,
        //         false,
        //         claim_funds,
        //     );

        //     quote_lots_released += matching_engine_response.num_quote_lots_out;
        //     base_lots_released += matching_engine_response.num_base_lots_out;

        //     if resting_order.num_base_lots == BaseLots::ZERO {
        //         manager.remove_order(slot_storage, market_state);
        //     } else {
        //         resting_order.write_to_slot(slot_storage, &order_id)?;
        //     }
        // } else if revert_if_fail {
        //     // When order is not present
        //     return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        // }
    }

    manager.write_prepared_indices(slot_storage, market_state);

    Ok(MatchingEngineResponse::new_withdraw(
        base_lots_released,
        quote_lots_released,
    ))
}
