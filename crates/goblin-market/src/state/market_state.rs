use crate::quantities::{QuoteLots, WrapperU64};

use super::{Side, SlotActions, SlotStorage, MARKET_STATE_KEY_SEED};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MarketState {
    /// Amount of fees collected from the market in its lifetime, in quote lots.
    pub collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    pub unclaimed_quote_lot_fees: QuoteLots,

    /// The number of active outer indices for bids
    pub bids_outer_indices: u16,

    /// The number of active outer indices for bids
    pub asks_outer_indices: u16,
    // 160 bits left, enough for best bid and best ask tick
    // alternative- just store inner_index for best bid and ask. This will only cost 2 bytes.
    // Former solution more efficient, reduces shifting.
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

impl MarketState {
    pub fn read_from_slot(slot_storage: &SlotStorage) -> Self {
        let slot = slot_storage.sload(&MARKET_SLOT_KEY);

        Self::decode(&slot)
    }

    pub fn decode(slot: &[u8; 32]) -> Self {
        MarketState {
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

#[cfg(test)]
mod test {
    use crate::quantities::{QuoteLots, WrapperU64};

    use super::MarketState;

    #[test]
    fn test_encode_and_decode_market_state() {
        let market = MarketState {
            collected_quote_lot_fees: QuoteLots::new(100),
            unclaimed_quote_lot_fees: QuoteLots::new(200),
            bids_outer_indices: 40,
            asks_outer_indices: 10,
        };

        let encoded = market.encode();
        let decoded_market = MarketState::decode(&encoded);

        assert_eq!(market, decoded_market);
    }
}
