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
    pub num_base_lots: BaseLots, // 63

    pub track_block: bool,                                  // 1
    pub last_valid_block_or_unix_timestamp_in_seconds: u32, // 32
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
    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        // Copy trader_address
        encoded_data[0..20].copy_from_slice(self.trader_address.as_slice());

        // Encode num_base_lots in big-endian format
        let num_base_lots_bytes = self.num_base_lots.as_u64().to_be_bytes();

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

    pub fn clear_order(&mut self) {
        self.trader_address = Address::ZERO;
        self.num_base_lots = BaseLots::ZERO;
        self.track_block = false;
        self.last_valid_block_or_unix_timestamp_in_seconds = 0;
    }

    pub fn does_not_exist(&self) -> bool {
        self.trader_address == Address::ZERO
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
    fn test_be_and_le() {
        let num = 1u64;
        println!("be {:?}", num.to_be_bytes());
        println!("le {:?}", num.to_le_bytes());
    }

    #[test]
    fn test_encode_resting_order() {
        let resting_order = SlotRestingOrder {
            trader_address: Address::ZERO,
            num_base_lots: BaseLots::new(1),
            track_block: true,
            last_valid_block_or_unix_timestamp_in_seconds: 257,
        };

        let encoded_order = resting_order.encode();
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
        let encoded_1 = resting_order_1.encode();

        assert_eq!(encoded_1[20], 0b0000_0000);
        assert_eq!(&encoded_1[21..28], [0u8; 7]);

        let resting_order_2 = SlotRestingOrder {
            trader_address: Address::ZERO,
            num_base_lots: BaseLots::new(0),
            track_block: true,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };
        let encoded_2 = resting_order_2.encode();

        assert_eq!(encoded_2[20], 0b1000_0000);
        assert_eq!(&encoded_2[21..28], [0u8; 7]);

        let resting_order_3 = SlotRestingOrder {
            trader_address: Address::ZERO,
            num_base_lots: BaseLots::new(9223372036854775807), // 2^63 - 1, max
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };
        let encoded_3 = resting_order_3.encode();

        assert_eq!(encoded_3[20], 0b0111_1111);
        assert_eq!(&encoded_3[21..28], [255u8; 7]);

        let resting_order_4 = SlotRestingOrder {
            trader_address: Address::ZERO,
            num_base_lots: BaseLots::new(9223372036854775807), // 2^63 - 1, max
            track_block: true,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };
        let encoded_4 = resting_order_4.encode();

        assert_eq!(encoded_4[20], 0b1111_1111);
        assert_eq!(&encoded_4[21..28], [255u8; 7]);
    }
}
