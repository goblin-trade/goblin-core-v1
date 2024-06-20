use alloc::vec::Vec;
use stylus_sdk::{
    alloy_primitives::{Address, B256},
    msg,
};

use crate::{
    program::{try_withdraw, GoblinResult},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, Ticks, WrapperU64},
    state::{
        MatchingEngine, MatchingEngineResponse, OrderId, RestingOrderIndex, SlotActions,
        SlotStorage,
    },
    GoblinMarket,
};

pub struct ReduceOrderPacket {
    pub order_id: OrderId,
    pub lots_to_remove: BaseLots,
}

impl ReduceOrderPacket {
    pub fn decode(bytes: &[u8; 32]) -> Self {
        ReduceOrderPacket {
            order_id: OrderId {
                price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
                resting_order_index: RestingOrderIndex::new(bytes[8]),
            },
            lots_to_remove: BaseLots::new(u64::from_be_bytes(bytes[9..16].try_into().unwrap())),
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
    let mut matching_engine = MatchingEngine {
        slot_storage: &mut SlotStorage::new(),
    };

    // State reads and writes are performed inside reduce_multiple_orders_inner()
    // The number of slot reads is dynamic
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = matching_engine.reduce_multiple_orders_inner(
        msg::sender(),
        order_packets,
        recipient.is_some(),
    )?;
    SlotStorage::storage_flush_cache(true);

    if let Some(recipient) = recipient {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);

        try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;
    }

    Ok(())
}
