use alloc::vec::Vec;
use stylus_sdk::{
    alloy_primitives::{Address, B256},
    msg,
};

use crate::{
    program::{try_withdraw, GoblinResult},
    quantities::{BaseAtomsRaw, QuoteAtomsRaw},
    state::{MatchingEngine, MatchingEngineResponse, SlotActions, SlotStorage},
    GoblinMarket,
};

/// TODO hack- we can have a single reduce_multiple_orders() function
/// This can be used to reduce or cancel multiple orders at once
pub fn process_cancel_multiple_orders_by_id(
    context: &mut GoblinMarket,
    order_ids: Vec<B256>,
    recipient: Option<Address>,
) -> GoblinResult<()> {
    let mut matching_engine = MatchingEngine {
        slot_storage: &mut SlotStorage::new(),
    };

    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = matching_engine
        .cancel_multiple_orders_by_id_inner(msg::sender(), order_ids, recipient.is_some())
        .unwrap_or_default();
    // TODO should throw error instead of default when response is None?
    // Look at None cases

    SlotStorage::storage_flush_cache(true);

    if let Some(recipient) = recipient {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);

        try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;
    }

    Ok(())
}
