use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{BaseLots, QuoteLots, WrapperU64},
    state::{
        MatchingEngineResponse, MutableBitmap, OrderId, Side, SlotActions, SlotRestingOrder,
        SlotStorage, TickIndices, TraderState, MARKET_STATE_KEY_SEED,
    },
};

use super::{Market, WritableMarket};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct FIFOMarket {
    /// Amount of fees collected from the market in its lifetime, in quote lots.
    collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    unclaimed_quote_lot_fees: QuoteLots,

    /// The number of active outer indices for bids
    bids_outer_indices: u16,

    /// The number of active outer indices for bids
    asks_outer_indices: u16,
}

const MARKET_SLOT_KEY: [u8; 32] = [
    MARKET_STATE_KEY_SEED,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

impl FIFOMarket {
    pub fn read_from_slot(slot_storage: &SlotStorage) -> Self {
        let slot = slot_storage.sload(&MARKET_SLOT_KEY);

        Self::decode(&slot)
    }

    pub fn decode(slot: &[u8; 32]) -> Self {
        FIFOMarket {
            collected_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[0..8].try_into().unwrap(),
            )),
            unclaimed_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[8..16].try_into().unwrap(),
            )),
            bids_outer_indices: u16::from_be_bytes(slot[16..18].try_into().unwrap()),
            asks_outer_indices: u16::from_be_bytes(slot[18..20].try_into().unwrap()),
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        encoded_data[0..8].copy_from_slice(&self.collected_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[8..16].copy_from_slice(&self.unclaimed_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[16..18].copy_from_slice(&self.bids_outer_indices.to_be_bytes());
        encoded_data[18..20].copy_from_slice(&self.asks_outer_indices.to_be_bytes());

        encoded_data
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage) {
        slot_storage.sstore(&MARKET_SLOT_KEY, &self.encode());
    }

    pub fn outer_index_length(&self, side: Side) -> u16 {
        if side == Side::Bid {
            self.bids_outer_indices
        } else {
            self.asks_outer_indices
        }
    }

    pub fn set_outer_index_length(&mut self, side: Side, value: u16) {
        if side == Side::Bid {
            self.bids_outer_indices = value;
        } else {
            self.asks_outer_indices = value;
        }
    }
}

impl Market for FIFOMarket {
    fn get_collected_fee_amount(&self) -> QuoteLots {
        self.collected_quote_lot_fees
    }

    fn get_uncollected_fee_amount(&self) -> QuoteLots {
        self.unclaimed_quote_lot_fees
    }
}

impl WritableMarket for FIFOMarket {}
