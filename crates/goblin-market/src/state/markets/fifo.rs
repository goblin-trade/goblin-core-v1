use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::{BaseLots, QuoteLots, WrapperU64},
    state::{
        slot_storage, MatchingEngineResponse, SlotActions, SlotStorage, TraderId, TraderState,
        MARKET_STATE_KEY_SEED,
    },
};

use super::{Market, WritableMarket};

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

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage) {
        slot_storage.sstore(&MARKET_SLOT_KEY, &self.encode());
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

impl WritableMarket for FIFOMarket {
    fn claim_funds(
        &self,
        slot_storage: &mut SlotStorage,
        trader: TraderId,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> Option<MatchingEngineResponse> {
        // Book not initialized
        if self.get_sequence_number() == 0 {
            return None;
        }
        let (quote_lots_received, base_lots_received) = {
            let mut trader_state = FIFOMarket::get_trader_state(slot_storage, trader);

            let quote_lots_free = num_quote_lots.min(trader_state.quote_lots_free);
            let base_lots_free = num_base_lots.min(trader_state.base_lots_free);

            // Update and write to slot
            trader_state.quote_lots_free -= quote_lots_free;
            trader_state.base_lots_free -= base_lots_free;
            trader_state.write_to_slot(slot_storage, trader);

            (quote_lots_free, base_lots_free)
        };

        Some(MatchingEngineResponse::new_withdraw(
            base_lots_received,
            quote_lots_received,
        ))
    }

    fn collect_fees(&mut self, slot_storage: &mut SlotStorage) -> QuoteLots {
        let quote_lot_fees = self.unclaimed_quote_lot_fees;

        // Mark as claimed
        self.collected_quote_lot_fees += self.unclaimed_quote_lot_fees;
        self.unclaimed_quote_lot_fees = QuoteLots::ZERO;

        // Write to slot
        self.write_to_slot(slot_storage);

        quote_lot_fees
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
