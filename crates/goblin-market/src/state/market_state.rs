use crate::{
    program::{ExceedTickSize, GoblinError, GoblinResult},
    quantities::{QuoteLots, Ticks, WrapperU64, MAX_TICK},
    require,
};

use super::{ArbContext, ContextActions, Side, MARKET_STATE_KEY_SEED};

#[repr(C)]
#[derive(Default, Debug, PartialEq)]
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

pub struct MarketPrices {
    pub best_bid_price: Ticks,
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
    pub fn read_from_slot(slot_storage: &ArbContext) -> Self {
        let slot = slot_storage.sload(&MARKET_SLOT_KEY);

        Self::decode(&slot)
    }

    pub fn decode(slot: &[u8; 32]) -> Self {
        MarketState {
            // TODO use 32 bits for collected and uncollected fees?
            // We could have some roll-over mechanism. Else if u32 will take long
            // enough we can stick to 32 bits.
            collected_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[0..8].try_into().unwrap(),
            )),
            unclaimed_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[8..16].try_into().unwrap(),
            )),
            bids_outer_indices: u16::from_be_bytes(slot[16..18].try_into().unwrap()),
            asks_outer_indices: u16::from_be_bytes(slot[18..20].try_into().unwrap()),

            // Question- default values for empty market?
            // Reading empty slot will yield 0.
            // When bids_outer_indices or asks_outer_indices is 0, we ignore these values

            // TODO use default values for best market prices-
            // Ticks::MIN for bid, Ticks::MAX for asks

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
            // If resting order index (0-7) were stored, we need 3 * 2 = 6 bits
            // but only 4 are free
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

        // TODO best prices only change during insertions and removals
        // Externally ensure that tick values during insertion are within bounds.
        // This check below can be removed
        require!(
            best_bid_price <= MAX_TICK && best_ask_price <= MAX_TICK,
            GoblinError::ExceedTickSize(ExceedTickSize {})
        );

        encoded_data[20..24].copy_from_slice(&(best_bid_price as u32).to_be_bytes());
        encoded_data[24..28].copy_from_slice(&(best_ask_price as u32).to_be_bytes());

        Ok(encoded_data)
    }

    pub fn write_to_slot(&self, slot_storage: &mut ArbContext) -> GoblinResult<()> {
        slot_storage.sstore(&MARKET_SLOT_KEY, &self.encode()?);

        Ok(())
    }

    pub fn outer_index_count(&self, side: Side) -> u16 {
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

    pub fn best_price(&self, side: Side) -> Ticks {
        if side == Side::Bid {
            self.best_bid_price
        } else {
            self.best_ask_price
        }
    }

    pub fn get_prices(&self) -> MarketPrices {
        debug_assert!(
            self.best_ask_price > self.best_bid_price,
            "Best ask price must be greater than best bid price"
        );

        MarketPrices {
            best_bid_price: self.best_bid_price,
            best_ask_price: self.best_ask_price,
        }
    }

    /// Update the best price
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `price_in_ticks`
    ///
    pub fn set_best_price(&mut self, side: Side, price: Ticks) {
        if side == Side::Bid {
            self.best_bid_price = price;
        } else {
            self.best_ask_price = price;
        }
    }

    /// Update the best price if the new price is closer to the centre
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `price_in_ticks`
    ///
    pub fn try_set_best_price(&mut self, side: Side, price_in_ticks: Ticks) {
        if side == Side::Bid && price_in_ticks > self.best_bid_price {
            self.best_bid_price = price_in_ticks;
        }
        if side == Side::Ask && price_in_ticks < self.best_ask_price {
            self.best_ask_price = price_in_ticks;
        }
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
