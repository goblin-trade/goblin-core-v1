use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::QuoteLots,
    state::{SlotStorage, TraderState},
};

pub trait RestingOrder {
    fn size(&self) -> u64;
    fn last_valid_block(&self) -> Option<u32>;
    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32>;
    fn is_expired(&self, current_slot: u32, current_unix_timestamp_in_seconds: u32) -> bool;
}

pub trait Market {
    fn get_collected_fee_amount(&self) -> QuoteLots;

    fn get_uncollected_fee_amount(&self) -> QuoteLots;

    fn get_sequence_number(&self) -> u64;

    fn get_trader_state(slot_storage: &SlotStorage, address: Address) -> TraderState;
}

pub trait WritableMarket {}
