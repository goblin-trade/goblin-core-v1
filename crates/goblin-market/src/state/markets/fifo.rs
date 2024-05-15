use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::{QuoteLots, WrapperU64},
    state::{SlotActions, SlotStorage, TraderState, MARKET_STATE_KEY_SEED},
};

use super::Market;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct FIFOMarket {
    /// The sequence number of the next event.
    order_sequence_number: u64,

    /// Amount of fees collected from the market in its lifetime, in quote lots.
    collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    unclaimed_quote_lot_fees: QuoteLots,
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
            order_sequence_number: u64::from_be_bytes(slot[0..8].try_into().unwrap()),
            collected_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[8..16].try_into().unwrap(),
            )),
            unclaimed_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[16..24].try_into().unwrap(),
            )),
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        encoded_data[0..8].copy_from_slice(&self.order_sequence_number.to_be_bytes());
        encoded_data[8..16].copy_from_slice(&self.collected_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[16..24].copy_from_slice(&self.unclaimed_quote_lot_fees.as_u64().to_be_bytes());

        encoded_data
    }
}

impl Market for FIFOMarket {
    fn get_collected_fee_amount(&self) -> QuoteLots {
        self.collected_quote_lot_fees
    }

    fn get_uncollected_fee_amount(&self) -> QuoteLots {
        self.unclaimed_quote_lot_fees
    }

    fn get_sequence_number(&self) -> u64 {
        self.order_sequence_number
    }

    fn get_trader_state(slot_storage: &SlotStorage, address: Address) -> TraderState {
        TraderState::read_from_slot(slot_storage, address)
    }
}

#[cfg(test)]
mod test {
    use crate::quantities::{QuoteLots, WrapperU64};

    use super::FIFOMarket;

    #[test]
    fn test_encode_and_decode_market_state() {
        let market = FIFOMarket {
            order_sequence_number: 1,
            collected_quote_lot_fees: QuoteLots::new(100),
            unclaimed_quote_lot_fees: QuoteLots::new(200),
        };

        let encoded = market.encode();
        let decoded_market = FIFOMarket::decode(&encoded);

        assert_eq!(market, decoded_market);
    }
}
