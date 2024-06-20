use crate::{
    program::{ExceedTickSize, GoblinError, GoblinResult},
    quantities::{QuoteLots, Ticks, WrapperU64, MAX_TICK},
    require,
};

use super::{
    BitmapGroup, IndexList, Side, SlotActions, SlotStorage, TickIndices, MARKET_STATE_KEY_SEED,
};

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

    // These are encoded as u32. In practice they only need 21 bits, so this can be optimized
    /// Price of the highest bid
    pub best_bid_price: Ticks,

    /// The lowest ask
    pub best_ask_price: Ticks,
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

            // Tick: u21 was encoded in 20..23 in big endian
            // Empty values to the left (LSB) in big endian
            best_bid_price: Ticks::new(
                // u64::from_be_bytes([0, 0, 0, 0, 0, slot[20], slot[21], slot[22]])
                u32::from_be_bytes(slot[20..24].try_into().unwrap()) as u64,
            ),

            best_ask_price: Ticks::new(
                // u64::from_be_bytes([0, 0, 0, 0, 0, slot[23], slot[24], slot[25]])
                u32::from_be_bytes(slot[24..28].try_into().unwrap()) as u64,
            ),
        }
    }

    pub fn encode(&self) -> Result<[u8; 32], GoblinError> {
        let mut encoded_data = [0u8; 32];

        encoded_data[0..8].copy_from_slice(&self.collected_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[8..16].copy_from_slice(&self.unclaimed_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[16..18].copy_from_slice(&self.bids_outer_indices.to_be_bytes());
        encoded_data[18..20].copy_from_slice(&self.asks_outer_indices.to_be_bytes());

        let best_bid_price = self.best_bid_price.as_u64();
        let best_ask_price = self.best_ask_price.as_u64();

        require!(
            best_bid_price <= MAX_TICK && best_ask_price <= MAX_TICK,
            GoblinError::ExceedTickSize(ExceedTickSize {})
        );

        encoded_data[20..24].copy_from_slice(&(best_bid_price as u32).to_be_bytes());
        encoded_data[24..28].copy_from_slice(&(best_ask_price as u32).to_be_bytes());

        Ok(encoded_data)
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage) -> GoblinResult<()> {
        slot_storage.sstore(&MARKET_SLOT_KEY, &self.encode()?);

        Ok(())
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

    /// Update the best price for a bid or ask
    ///
    /// The current best price is found by reading the outer index from index_list and
    /// the inner index from the bitmap corresponding to this outer index
    ///
    /// # Arguments
    ///
    /// * `index_list`
    /// * `slot_storage`
    ///
    pub fn update_best_price(&mut self, index_list: &IndexList, slot_storage: &SlotStorage) {
        let best_price = if index_list.side == Side::Bid {
            &mut self.best_bid_price
        } else {
            &mut self.best_ask_price
        };

        // 1- get new outer index
        let new_outer_index = index_list.get_best_outer_index(slot_storage);

        let TickIndices {
            outer_index: old_outer_index,
            inner_index: old_inner_index,
        } = best_price.to_indices();

        let bitmap_group = BitmapGroup::new_from_slot(slot_storage, &new_outer_index);

        // If outer index did not change, lookup in the same bitmap group starting from
        // the old inner index
        let previous_best_inner_index = if new_outer_index == old_outer_index {
            Some(old_inner_index)
        } else {
            // If the best_outer_index has changed, this is not needed
            None
        };

        // 2- get new inner index
        // This should not return None because active orders are guaranteed in bitmap groups with active
        // outer indices
        let new_inner_index = bitmap_group
            .best_active_index(index_list.side.clone(), previous_best_inner_index)
            .unwrap();

        // 3- update best price
        *best_price = Ticks::from_indices(new_outer_index, new_inner_index);
    }
}

#[cfg(test)]
mod test {
    use crate::quantities::{QuoteLots, Ticks, WrapperU64};

    use super::MarketState;

    #[test]
    fn test_encode_and_decode_market_state() {
        let market = MarketState {
            collected_quote_lot_fees: QuoteLots::new(100),
            unclaimed_quote_lot_fees: QuoteLots::new(200),
            bids_outer_indices: 40,
            asks_outer_indices: 10,
            best_bid_price: Ticks::new(40),
            best_ask_price: Ticks::new(50),
        };

        let encoded = market.encode().unwrap();
        let decoded_market = MarketState::decode(&encoded);

        assert_eq!(market, decoded_market);
    }
}
