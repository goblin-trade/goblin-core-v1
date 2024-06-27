use alloc::vec::Vec;
use stylus_sdk::{
    alloy_primitives::{Address, B256},
    msg,
};

use crate::{
    program::{try_withdraw, GoblinResult},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, Ticks, WrapperU64},
    state::{
        market_state, MarketState, MatchingEngine, MatchingEngineResponse, OrderId,
        RestingOrderIndex, SlotActions, SlotStorage,
    },
    GoblinMarket,
};

pub struct ReduceOrderPacket {
    // ID of order to reduce
    pub order_id: OrderId,

    // Reduce at most these many lots. Pass u64::MAX to close
    pub lots_to_remove: BaseLots,

    // Revert entire TX if reduction fails for this order
    pub revert_if_fail: bool,
}

impl ReduceOrderPacket {
    pub fn decode(bytes: &[u8; 32]) -> Self {
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

/// Try to reduce one or more resting orders. Also used to cancel orders.
///
/// # Arguments
///
/// * `context`
/// * `order_packets`
/// * `recipient` - Transfer claimed funds to this address
///
pub fn process_reduce_multiple_orders(
    context: &mut GoblinMarket,
    order_packets: Vec<B256>,
    recipient: Option<Address>,
) -> GoblinResult<()> {
    let slot_storage = &mut SlotStorage::new();

    let market_state = &mut MarketState::read_from_slot(slot_storage);

    let mut matching_engine = MatchingEngine { slot_storage };

    // State reads and writes are performed inside reduce_multiple_orders_inner()
    // The number of slot reads is dynamic
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = matching_engine.reduce_multiple_orders_inner(
        market_state,
        msg::sender(),
        order_packets,
        recipient.is_some(),
    )?;
    market_state.write_to_slot(slot_storage)?;
    SlotStorage::storage_flush_cache(true);

    if let Some(recipient) = recipient {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);

        try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;
    }

    Ok(())
}
