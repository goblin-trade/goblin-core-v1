use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::{BaseLots, Ticks, WrapperU64},
    state::{
        slot_storage::SlotKey, RestingOrder, SlotActions, SlotStorage, ORDERS_PER_TICK,
        RESTING_ORDER_KEY_SEED,
    },
};

#[derive(Clone)]
#[repr(transparent)]
pub struct RestingOrderIndex {
    inner: u8,
}

impl RestingOrderIndex {
    pub fn new(inner: u8) -> Self {
        assert!(inner < ORDERS_PER_TICK);
        RestingOrderIndex { inner }
    }

    pub fn as_u8(&self) -> u8 {
        self.inner
    }
}

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

/// Resting order on a 32 byte slot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlotRestingOrder {
    pub trader_address: Address, // 20 bytes = 160 bits
    pub num_base_lots: BaseLots, // 64
    // use a bool to track if last_valid_block is used. If last_valid_block is 0 then this is none.
    // this leaves us 256 - 160 - 64 - 1 = 31 bits for block / timestamp
    pub last_valid_block: u32,
    pub last_valid_unix_timestamp_in_seconds: u32,
}

impl SlotRestingOrder {
    pub fn new_default(trader_address: Address, num_base_lots: BaseLots) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_block: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new(
        trader_address: Address,
        num_base_lots: BaseLots,
        last_valid_slot: Option<u32>,
        last_valid_unix_timestamp_in_seconds: Option<u32>,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_block: last_valid_slot.unwrap_or(0),
            last_valid_unix_timestamp_in_seconds: last_valid_unix_timestamp_in_seconds.unwrap_or(0),
        }
    }

    pub fn new_with_last_valid_slot(
        trader_address: Address,
        num_base_lots: BaseLots,
        last_valid_slot: u32,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_block: last_valid_slot,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new_with_last_valid_unix_timestamp(
        trader_address: Address,
        num_base_lots: BaseLots,
        last_valid_unix_timestamp_in_seconds: u32,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_block: 0,
            last_valid_unix_timestamp_in_seconds,
        }
    }

    /// Decode from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        let trader_address = Address::from_slice(&slot[0..20]);

        let num_base_lots =
            BaseLots::new(u32::from_be_bytes(slot[20..24].try_into().unwrap()) as u64);
        let last_valid_block = u32::from_be_bytes(slot[24..28].try_into().unwrap());
        let last_valid_unix_timestamp_in_seconds =
            u32::from_be_bytes(slot[28..32].try_into().unwrap());

        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_block,
            last_valid_unix_timestamp_in_seconds,
        }
    }

    /// Encode as a 32 bit slot in big endian
    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        // Copy trader_address
        encoded_data[0..20].copy_from_slice(self.trader_address.as_slice());

        // num_base_lots is encoded as u32. Ensure that it fits
        let num_base_lots = self.num_base_lots.as_u64();
        assert!(num_base_lots <= u32::MAX as u64);

        // Encode num_base_lots in big-endian format
        let num_base_lots_bytes = (num_base_lots as u32).to_be_bytes();
        encoded_data[20..24].copy_from_slice(&num_base_lots_bytes);

        // Encode last_valid_block in big-endian format
        let last_valid_block_bytes = self.last_valid_block.to_be_bytes();
        encoded_data[24..28].copy_from_slice(&last_valid_block_bytes);

        // Encode last_valid_unix_timestamp_in_seconds in big-endian format
        let last_valid_unix_timestamp_bytes =
            self.last_valid_unix_timestamp_in_seconds.to_be_bytes();
        encoded_data[28..32].copy_from_slice(&last_valid_unix_timestamp_bytes);

        encoded_data
    }

    /// Load CBRestingOrder from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &OrderId) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        SlotRestingOrder::decode(slot)
    }

    /// Encode and save CBRestingOrder to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &OrderId) {
        let encoded = self.encode();

        slot_storage.sstore(&key.get_key(), &encoded);
    }
}

impl RestingOrder for SlotRestingOrder {
    fn size(&self) -> u64 {
        self.num_base_lots.as_u64()
    }

    fn last_valid_block(&self) -> Option<u32> {
        if self.last_valid_block == 0 {
            None
        } else {
            Some(self.last_valid_block)
        }
    }

    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32> {
        if self.last_valid_unix_timestamp_in_seconds == 0 {
            None
        } else {
            Some(self.last_valid_unix_timestamp_in_seconds)
        }
    }

    fn is_expired(&self, current_slot: u32, current_unix_timestamp_in_seconds: u32) -> bool {
        (self.last_valid_block != 0 && self.last_valid_block < current_slot)
            || (self.last_valid_unix_timestamp_in_seconds != 0
                && self.last_valid_unix_timestamp_in_seconds < current_unix_timestamp_in_seconds)
    }
}

#[cfg(test)]
mod test {
    use stylus_sdk::alloy_primitives::{address, Address};

    use crate::quantities::{BaseLots, WrapperU64};

    use super::SlotRestingOrder;

    #[test]
    fn test_encode_resting_order() {
        let resting_order = SlotRestingOrder {
            trader_address: Address::ZERO.0.into(),
            num_base_lots: BaseLots::new(1),
            last_valid_block: 256,
            last_valid_unix_timestamp_in_seconds: 257,
        };

        let encoded_order = resting_order.encode();
        assert_eq!(
            encoded_order,
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 1
                0, 0, 1, 0, // 256
                0, 0, 1, 1, // 257
            ]
        );
    }

    #[test]
    fn test_decode_resting_order() {
        let slot: [u8; 32] = [
            // 0x0000000000000000000000000000000000000001
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, // 1
            0, 0, 0, 0, // 0
            0, 0, 1, 1, // 256
        ];

        let resting_order = SlotRestingOrder::decode(slot);

        let expected_address = address!("0000000000000000000000000000000000000001");
        assert_eq!(resting_order.trader_address, expected_address);
        assert_eq!(resting_order.num_base_lots, 1);
        assert_eq!(resting_order.last_valid_block, 0);
        assert_eq!(resting_order.last_valid_unix_timestamp_in_seconds, 257);
    }
}
