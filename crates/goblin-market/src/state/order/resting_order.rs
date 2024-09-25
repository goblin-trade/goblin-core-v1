use core::ops::AddAssign;

use stylus_sdk::alloy_primitives::{address, Address};

use crate::{
    program::{
        compute_quote_lots, types::matching_engine_response::MatchingEngineResponse,
        ExceedRestingOrderSize, GoblinError,
    },
    quantities::{BaseLots, QuoteLots, Ticks, WrapperU64},
    require,
    state::{ArbContext, ContextActions, Side, SlotKey, TraderState},
};

use super::order_id::OrderId;

const NULL_ADDRESS: Address = address!("0000000000000000000000000000000000000001");

/// Resting order on a 32 byte slot
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SlotRestingOrder {
    pub trader_address: Address, // 20 bytes = 160 bits
    pub num_base_lots: BaseLots, // 63

    pub track_block: bool,                                  // 1
    pub last_valid_block_or_unix_timestamp_in_seconds: u32, // 32
}

impl AddAssign for SlotRestingOrder {
    /// Adds the `num_base_lots` of another `SlotRestingOrder` to this one.
    ///
    /// # Safety
    /// You must ensure externally that both `SlotRestingOrder` instances
    /// have the same `trader_address` and `last_valid_block_or_unix_timestamp_in_seconds`.
    ///
    /// # Arguments
    /// * `other` - Another `SlotRestingOrder` whose `num_base_lots` will be added.
    fn add_assign(&mut self, other: Self) {
        // External validation required for address and expiry equality
        self.num_base_lots += other.num_base_lots;
    }
}

impl SlotRestingOrder {
    pub fn new_default(trader_address: Address, num_base_lots: BaseLots) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new(
        trader_address: Address,
        num_base_lots: BaseLots,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
        }
    }

    /// Decode from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        let trader_address = Address::from_slice(&slot[0..20]);

        let num_base_lots = BaseLots::new(u64::from_be_bytes([
            slot[20] & 0b0111_1111,
            slot[21],
            slot[22],
            slot[23],
            slot[24],
            slot[25],
            slot[26],
            slot[27],
        ]));

        let track_timestamp = (slot[20] & 0b1000_0000) != 0;

        let last_valid_block_or_unix_timestamp_in_seconds =
            u32::from_be_bytes([slot[28], slot[29], slot[30], slot[31]]);

        SlotRestingOrder {
            trader_address,
            num_base_lots,
            track_block: track_timestamp,
            last_valid_block_or_unix_timestamp_in_seconds,
        }
    }

    /// Encode as a 32 bit slot in big endian
    pub fn encode(&self) -> Result<[u8; 32], GoblinError> {
        let mut encoded_data = [0u8; 32];

        // Copy trader_address
        encoded_data[0..20].copy_from_slice(self.trader_address.as_slice());

        // Encode num_base_lots in big-endian format
        let num_base_lots_bytes = self.num_base_lots.as_u64().to_be_bytes();

        // ensure that num_base_lots is less than or equal to 2^63 - 1
        // optimization- check LSB is 0 instead of doing a comparison operation
        require!(
            num_base_lots_bytes[0] & 0b1000_0000 == 0,
            GoblinError::ExceedRestingOrderSize(ExceedRestingOrderSize {})
        );

        encoded_data[20..28].copy_from_slice(&num_base_lots_bytes);

        // Encode track_timestamp flag in the LSB of the i=20 byte
        if self.track_block {
            encoded_data[20] |= 0b1000_0000;
        }

        // Encode last_valid_block_or_unix_timestamp_in_seconds in big-endian format
        encoded_data[28..32].copy_from_slice(
            &self
                .last_valid_block_or_unix_timestamp_in_seconds
                .to_be_bytes(),
        );

        Ok(encoded_data)
    }

    /// Load CBRestingOrder from slot storage
    pub fn new_from_slot(slot_storage: &ArbContext, key: OrderId) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        SlotRestingOrder::decode(slot)
    }

    pub fn new_from_raw_key(slot_storage: &ArbContext, key: &[u8; 32]) -> Self {
        let slot = slot_storage.sload(key);

        SlotRestingOrder::decode(slot)
    }

    /// Encode and save CBRestingOrder to slot
    pub fn write_to_slot(
        &self,
        slot_storage: &mut ArbContext,
        key: &OrderId,
    ) -> Result<(), GoblinError> {
        let encoded = self.encode()?;
        slot_storage.sstore(&key.get_key(), &encoded);

        Ok(())
    }

    /// Adds the `num_base_lots` of another `SlotRestingOrder` to this one.
    ///
    /// # Safety
    /// This function assumes that both `SlotRestingOrder` instances have the same `trader_address`
    /// and `last_valid_block_or_unix_timestamp_in_seconds`. It does not check these fields.
    ///
    /// # Arguments
    /// * `other` - A reference to the other `SlotRestingOrder` whose `num_base_lots` will be added.
    pub fn merge_order(&mut self, other: &SlotRestingOrder) {
        // External validation required for address and expiry equality
        self.num_base_lots += other.num_base_lots;
    }

    // TODO remove. No need to write cleared resting orders to slot, let them be.
    // Updating bitmaps is enough.
    // TODO update match_order()
    pub fn clear_order(&mut self) {
        // Gas optimization- set address to 0x1. This way the slot is not cleared
        self.trader_address = NULL_ADDRESS;
        self.num_base_lots = BaseLots::ZERO;
        self.track_block = false;
        self.last_valid_block_or_unix_timestamp_in_seconds = 0;
    }

    // The order slot was never initialized or was cleared
    // TODO remove. Use bitmap to discover whether open or closed
    pub fn does_not_exist(&self) -> bool {
        self.trader_address == Address::ZERO || self.trader_address == NULL_ADDRESS
    }

    pub fn is_expired(&self, current_block: u32, current_unix_timestamp_in_seconds: u32) -> bool {
        if self.last_valid_block_or_unix_timestamp_in_seconds == 0 {
            return false;
        }

        (self.track_block && current_block > self.last_valid_block_or_unix_timestamp_in_seconds)
            || (!self.track_block
                && current_unix_timestamp_in_seconds
                    > self.last_valid_block_or_unix_timestamp_in_seconds)
    }

    /// Whether the order is empty and can be removed from the book.
    ///
    /// Empty orders are not written to slot. Only their corresponding bit is remove
    /// from the bitmap.
    pub fn is_empty(&self) -> bool {
        self.num_base_lots == BaseLots::ZERO
    }

    /// Reduce a resting order.
    ///
    /// Updates order and trader states, but doesn't write. Perform write externally.
    ///
    /// # Arguments
    ///
    /// * `trader_state`
    /// * `trader`
    /// * `side`
    /// * `price_in_ticks`
    /// * `lots_to_remove` - Try to reduce size by this many lots. Pass u64::MAX to close entire order
    /// * `order_is_expired`
    /// * `claim_funds`
    ///
    /// # Returns
    ///
    /// Number of base and quote lots released after reducing the order.
    ///
    pub fn reduce_order(
        &mut self,
        trader_state: &mut TraderState,
        side: Side,
        price_in_ticks: Ticks,
        lots_to_remove: BaseLots,
        order_is_expired: bool,
        claim_funds: bool,
    ) -> MatchingEngineResponse {
        let base_lots_to_remove = if order_is_expired {
            // If the order is tagged as expired, remove all of the base lots
            self.num_base_lots
        } else {
            self.num_base_lots.min(lots_to_remove)
        };

        // Deduct lots from resting order state
        self.num_base_lots -= base_lots_to_remove;

        // EMIT ExpiredOrder / Reduce

        // Free up tokens from trader state
        let (num_quote_lots, num_base_lots) = {
            match side {
                Side::Bid => {
                    // A bid order consists of locked up 'quote tokens' bidding to buy the base token.
                    // Quote tokens are released on reducing the order.
                    let quote_lots = compute_quote_lots(price_in_ticks, base_lots_to_remove);

                    trader_state.unlock_quote_lots(quote_lots);
                    (quote_lots, BaseLots::ZERO)
                }
                Side::Ask => {
                    // A bid order consists of locked up 'base tokens' bidding to sell the base token
                    // in exchange of the quote token.
                    // Base tokens are released on reducing the order.
                    trader_state.unlock_base_lots(base_lots_to_remove);
                    (QuoteLots::ZERO, base_lots_to_remove)
                }
            }
        };

        // We don't want to claim funds if an order is removed from the book during a self trade
        // or if the user specifically indicates that they don't want to claim funds.
        // TODO test claim_funds_inner()
        if claim_funds {
            trader_state.claim_funds(num_quote_lots, num_base_lots)
        } else {
            // No claim case- the order is reduced but no funds will be claimed
            MatchingEngineResponse::default()
        }
    }
}

pub trait RestingOrder {
    fn size(&self) -> u64;
    fn last_valid_block(&self) -> Option<u32>;
    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32>;
}

impl RestingOrder for SlotRestingOrder {
    fn size(&self) -> u64 {
        self.num_base_lots.as_u64()
    }

    fn last_valid_block(&self) -> Option<u32> {
        if self.track_block && self.last_valid_block_or_unix_timestamp_in_seconds != 0 {
            Some(self.last_valid_block_or_unix_timestamp_in_seconds)
        } else {
            None
        }
    }

    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32> {
        if !self.track_block && self.last_valid_block_or_unix_timestamp_in_seconds != 0 {
            Some(self.last_valid_block_or_unix_timestamp_in_seconds)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_encoding {
        use super::*;

        #[test]
        fn test_highest_valid_base_lot_size() {
            let resting_order = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(9223372036854775807),
                track_block: false,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };

            resting_order.encode().unwrap();
        }

        #[test]
        #[should_panic]
        fn test_base_lot_size_overflow() {
            let resting_order = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(9223372036854775807 + 1),
                track_block: false,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };

            resting_order.encode().unwrap();
        }

        #[test]
        fn test_encode_resting_order() {
            let resting_order = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(1),
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 257,
            };

            let encoded_order = resting_order.encode().unwrap();
            assert_eq!(
                encoded_order,
                [
                    // address- 0
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
                    // num_base_lots- 1, track_block true
                    0b1000_0000,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    // 257
                    0,
                    0,
                    1,
                    1,
                ]
            );

            let decoded_order = SlotRestingOrder::decode(encoded_order);

            assert_eq!(resting_order.trader_address, decoded_order.trader_address);
            assert_eq!(resting_order.num_base_lots, decoded_order.num_base_lots);
            assert_eq!(resting_order.track_block, decoded_order.track_block);
            assert_eq!(
                resting_order.last_valid_block_or_unix_timestamp_in_seconds,
                decoded_order.last_valid_block_or_unix_timestamp_in_seconds
            );
        }

        #[test]
        fn test_decode_resting_order() {
            let slot: [u8; 32] = [
                // address- 0x000...1
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
                1,
                // track_block false, max lots
                0b0111_1111,
                255,
                255,
                255,
                255,
                255,
                255,
                255,
                0,
                0,
                1,
                1, // 257
            ];

            let resting_order = SlotRestingOrder::decode(slot);

            let expected_address = address!("0000000000000000000000000000000000000001");
            assert_eq!(resting_order.trader_address, expected_address);
            assert_eq!(
                resting_order.num_base_lots,
                BaseLots::new(9223372036854775807)
            );

            assert_eq!(resting_order.track_block, false);
            assert_eq!(
                resting_order.last_valid_block_or_unix_timestamp_in_seconds,
                257
            );
        }

        #[test]
        fn test_track_block_encoding() {
            let resting_order_1 = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(0),
                track_block: false,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };
            let encoded_1 = resting_order_1.encode().unwrap();

            assert_eq!(encoded_1[20], 0b0000_0000);
            assert_eq!(&encoded_1[21..28], [0u8; 7]);

            let resting_order_2 = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(0),
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };
            let encoded_2 = resting_order_2.encode().unwrap();

            assert_eq!(encoded_2[20], 0b1000_0000);
            assert_eq!(&encoded_2[21..28], [0u8; 7]);

            let resting_order_3 = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(9223372036854775807), // 2^63 - 1, max
                track_block: false,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };
            let encoded_3 = resting_order_3.encode().unwrap();

            assert_eq!(encoded_3[20], 0b0111_1111);
            assert_eq!(&encoded_3[21..28], [255u8; 7]);

            let resting_order_4 = SlotRestingOrder {
                trader_address: Address::ZERO,
                num_base_lots: BaseLots::new(9223372036854775807), // 2^63 - 1, max
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 0,
            };
            let encoded_4 = resting_order_4.encode().unwrap();

            assert_eq!(encoded_4[20], 0b1111_1111);
            assert_eq!(&encoded_4[21..28], [255u8; 7]);
        }
    }

    mod reduce_order {
        use super::*;

        #[test]
        fn test_reduce_for_bids() {
            let side = Side::Ask;
            let price_in_ticks = Ticks::ONE;
            let order_size = BaseLots::new(2);

            let mut trader_state = TraderState {
                quote_lots_locked: QuoteLots::ZERO,
                quote_lots_free: QuoteLots::ZERO,
                base_lots_locked: BaseLots::new(2),
                base_lots_free: BaseLots::ZERO,
            };
            let mut resting_order = SlotRestingOrder {
                trader_address: Address::default(),
                num_base_lots: order_size,
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 1,
            };

            let order_is_expired = false;
            let claim_funds = false;

            let lots_to_remove_0 = BaseLots::ONE;
            let response_0 = resting_order.reduce_order(
                &mut trader_state,
                side,
                price_in_ticks,
                lots_to_remove_0,
                order_is_expired,
                claim_funds,
            );
            assert_eq!(response_0, MatchingEngineResponse::default());
            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ZERO,
                    quote_lots_free: QuoteLots::ZERO,
                    base_lots_locked: BaseLots::ONE,
                    base_lots_free: lots_to_remove_0,
                }
            );

            let lots_to_remove_1 = BaseLots::ONE;
            let response_1 = resting_order.reduce_order(
                &mut trader_state,
                side,
                price_in_ticks,
                lots_to_remove_1,
                order_is_expired,
                claim_funds,
            );
            assert_eq!(response_1, MatchingEngineResponse::default());
            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ZERO,
                    quote_lots_free: QuoteLots::ZERO,
                    base_lots_locked: BaseLots::ZERO,
                    base_lots_free: BaseLots::new(2),
                }
            );
        }

        #[test]
        fn test_reduce_for_asks() {
            let side = Side::Bid;
            let price_in_ticks = Ticks::ONE;
            let order_size = BaseLots::new(2);
            let quote_lots_locked = QuoteLots::new(2);

            let mut trader_state = TraderState {
                quote_lots_locked,
                quote_lots_free: QuoteLots::ZERO,
                base_lots_locked: BaseLots::ZERO,
                base_lots_free: BaseLots::ZERO,
            };
            let mut resting_order = SlotRestingOrder {
                trader_address: Address::default(),
                num_base_lots: order_size,
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 1,
            };

            let order_is_expired = false;
            let claim_funds = false;

            let lots_to_remove_0 = BaseLots::ONE;
            let response_0 = resting_order.reduce_order(
                &mut trader_state,
                side,
                price_in_ticks,
                lots_to_remove_0,
                order_is_expired,
                claim_funds,
            );
            assert_eq!(response_0, MatchingEngineResponse::default());
            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ONE,
                    quote_lots_free: QuoteLots::ONE,
                    base_lots_locked: BaseLots::ZERO,
                    base_lots_free: BaseLots::ZERO,
                }
            );

            let lots_to_remove_1 = BaseLots::ONE;
            let response_1 = resting_order.reduce_order(
                &mut trader_state,
                side,
                price_in_ticks,
                lots_to_remove_1,
                order_is_expired,
                claim_funds,
            );
            assert_eq!(response_1, MatchingEngineResponse::default());
            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ZERO,
                    quote_lots_free: QuoteLots::new(2),
                    base_lots_locked: BaseLots::ZERO,
                    base_lots_free: BaseLots::ZERO,
                }
            );
        }

        #[test]
        fn test_all_lots_freed_for_expired_order() {
            let side = Side::Ask;
            let price_in_ticks = Ticks::ONE;
            let order_size = BaseLots::new(2);

            let mut trader_state = TraderState {
                quote_lots_locked: QuoteLots::ZERO,
                quote_lots_free: QuoteLots::ZERO,
                base_lots_locked: order_size,
                base_lots_free: BaseLots::ZERO,
            };
            let mut resting_order = SlotRestingOrder {
                trader_address: Address::default(),
                num_base_lots: order_size,
                track_block: true,
                last_valid_block_or_unix_timestamp_in_seconds: 1,
            };

            let order_is_expired = true;
            let claim_funds = false;

            let lots_to_remove = BaseLots::ONE;
            let response = resting_order.reduce_order(
                &mut trader_state,
                side,
                price_in_ticks,
                lots_to_remove,
                order_is_expired,
                claim_funds,
            );

            assert_eq!(response, MatchingEngineResponse::default());
            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ZERO,
                    quote_lots_free: QuoteLots::ZERO,
                    base_lots_locked: BaseLots::ZERO,
                    base_lots_free: order_size,
                }
            );
        }
    }
}
