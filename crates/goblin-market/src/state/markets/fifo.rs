use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::QuoteLots,
    state::{SlotStorage, TraderState},
};

use super::Market;

pub struct FIFOMarket {
    /// The sequence number of the next event.
    order_sequence_number: u64,

    /// Amount of fees collected from the market in its lifetime, in quote lots.
    collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    unclaimed_quote_lot_fees: QuoteLots,
}

impl Market for FIFOMarket {
    fn get_trader_state(slot_storage: &SlotStorage, address: Address) -> TraderState {
        TraderState::read_from_slot(slot_storage, address)
    }
}
