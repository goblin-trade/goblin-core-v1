use core::ops::{Add, AddAssign};

use stylus_sdk::alloy_primitives::{address, Address};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{ExceedRestingOrderSize, GoblinError},
    quantities::{BaseLots, QuoteLots, Ticks, WrapperU64},
    require,
    state::{
        slot_storage::SlotKey, MatchingEngineResponse, ReduceOrderInnerResponse, RestingOrder,
        Side, SlotActions, SlotStorage, TraderState, ORDERS_PER_TICK, RESTING_ORDER_KEY_SEED,
    },
};

const NULL_ADDRESS: Address = address!("0000000000000000000000000000000000000001");

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct RestingOrderIndex {
    inner: u8,
}

impl RestingOrderIndex {
    pub const ZERO: RestingOrderIndex = RestingOrderIndex { inner: 0 };
    pub const MIN: RestingOrderIndex = RestingOrderIndex::ZERO;
    pub const ONE: RestingOrderIndex = RestingOrderIndex { inner: 1 };
    pub const MAX: RestingOrderIndex = RestingOrderIndex { inner: 7 };

    pub fn new(inner: u8) -> Self {
        assert!(inner < ORDERS_PER_TICK);
        RestingOrderIndex { inner }
    }

    pub fn as_u8(&self) -> u8 {
        self.inner
    }
}

impl Add for RestingOrderIndex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RestingOrderIndex {
            inner: self.inner.wrapping_add(other.inner),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct OrderId {
    /// Tick where order is placed
    pub price_in_ticks: Ticks,

    /// Resting order index between 0 to 7. A single tick can have at most 8 orders
    pub resting_order_index: RestingOrderIndex,
}

impl SlotKey for OrderId {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = RESTING_ORDER_KEY_SEED;
        key[1..9].copy_from_slice(&self.price_in_ticks.as_u64().to_be_bytes());
        key[9] = self.resting_order_index.as_u8();

        key
    }
}

impl OrderId {
    pub fn decode(bytes: &[u8; 32]) -> Self {
        OrderId {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[1..9].try_into().unwrap())),
            resting_order_index: RestingOrderIndex::new(bytes[9]),
        }
    }

    /// Find the side of an active resting order (not a new order being placed)
    ///
    /// An active bid cannot have a price more than the best bid price,
    /// and an active ask cannot have a price lower than the best ask price.
    ///
    pub fn side(&self, best_bid_price: Ticks, best_ask_price: Ticks) -> Side {
        if self.price_in_ticks >= best_ask_price {
            Side::Ask
        } else if self.price_in_ticks <= best_bid_price {
            Side::Bid
        } else {
            // There are no active orders in the spread
            // However there could be activated slots. Ensure that they are not tested here.
            unreachable!()
        }
    }
}

/// Resting order on a 32 byte slot
#[repr(C)]
#[derive(Copy, Clone)]
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
    pub fn new_from_slot(slot_storage: &SlotStorage, key: OrderId) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        SlotRestingOrder::decode(slot)
    }

    pub fn new_from_raw_key(slot_storage: &SlotStorage, key: &[u8; 32]) -> Self {
        let slot = slot_storage.sload(key);

        SlotRestingOrder::decode(slot)
    }

    /// Encode and save CBRestingOrder to slot
    pub fn write_to_slot(
        &self,
        slot_storage: &mut SlotStorage,
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

    pub fn clear_order(&mut self) {
        // Gas optimization- set address to 0x1. This way the slot is not cleared
        self.trader_address = NULL_ADDRESS;
        self.num_base_lots = BaseLots::ZERO;
        self.track_block = false;
        self.last_valid_block_or_unix_timestamp_in_seconds = 0;
    }

    // The order slot was never initialized or was cleared
    pub fn does_not_exist(&self) -> bool {
        self.trader_address == Address::ZERO || self.trader_address == NULL_ADDRESS
    }

    pub fn expired(&self, current_block: u32, current_unix_timestamp_in_seconds: u32) -> bool {
        if self.last_valid_block_or_unix_timestamp_in_seconds == 0 {
            return false;
        }

        (self.track_block && current_block > self.last_valid_block_or_unix_timestamp_in_seconds)
            || (!self.track_block
                && current_unix_timestamp_in_seconds
                    > self.last_valid_block_or_unix_timestamp_in_seconds)
    }

    /// Try to reduce a resting order. Returns None if the order doesn't exist
    /// or belongs to another trader.
    ///
    /// Updates order and trader states, but doesn't write. Perform write externally.
    ///
    /// # Arguments
    ///
    /// * `trader_state`
    /// * `trader`
    /// * `order_id`
    /// * `side`
    /// * `lots_to_remove` - Try to reduce size by this many lots. Pass u64::MAX to close entire order
    /// * `order_is_expired`
    /// * `claim_funds`
    ///
    pub fn reduce_order(
        &mut self,
        trader_state: &mut TraderState,
        trader: Address,
        order_id: &OrderId,
        side: Side,
        lots_to_remove: BaseLots,
        order_is_expired: bool,
        claim_funds: bool,
    ) -> Option<ReduceOrderInnerResponse> {
        // Find lots to remove
        let (should_remove_order_from_book, base_lots_to_remove) = {
            // Order does not exist (blank slot), or belongs to another trader
            if self.trader_address != trader {
                return None;
            }

            // If the order is tagged as expired, we remove it from the book regardless of the size.
            if order_is_expired {
                (true, self.num_base_lots)
            } else {
                let base_lots_to_remove = self.num_base_lots.min(lots_to_remove);

                (
                    base_lots_to_remove == self.num_base_lots,
                    base_lots_to_remove,
                )
            }
        };

        // Mutate order
        let _base_lots_remaining = if should_remove_order_from_book {
            self.clear_order();

            BaseLots::ZERO
        } else {
            // Reduce order
            self.num_base_lots -= base_lots_to_remove;

            self.num_base_lots
        };

        // EMIT ExpiredOrder / Reduce

        // We don't want to claim funds if an order is removed from the book during a self trade
        // or if the user specifically indicates that they don't want to claim funds.
        if claim_funds {
            // Update trader state
            let (num_quote_lots, num_base_lots) = {
                match side {
                    Side::Bid => {
                        let quote_lots = (order_id.price_in_ticks
                            * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                            * base_lots_to_remove)
                            / BASE_LOTS_PER_BASE_UNIT;
                        trader_state.unlock_quote_lots(quote_lots);

                        (quote_lots, BaseLots::ZERO)
                    }
                    Side::Ask => {
                        trader_state.unlock_base_lots(base_lots_to_remove);

                        (QuoteLots::ZERO, base_lots_to_remove)
                    }
                }
            };

            Some(ReduceOrderInnerResponse {
                matching_engine_response: trader_state
                    .claim_funds_inner(num_quote_lots, num_base_lots),
                should_remove_order_from_book,
            })
        } else {
            // No claim case- the order is reduced but no funds will be claimed
            Some(ReduceOrderInnerResponse {
                matching_engine_response: MatchingEngineResponse::default(),
                should_remove_order_from_book,
            })
        }
    }
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

    // TODO is_expired() function
}

#[cfg(test)]
mod test {
    use stylus_sdk::alloy_primitives::{address, Address};

    use crate::quantities::{BaseLots, WrapperU64};

    use super::SlotRestingOrder;

    #[test]
    fn highest_valid_base_lot_size() {
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
    fn base_lot_size_overflow() {
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
