use crate::state::{slot_storage::SlotKey, RestingOrder, SlotActions, SlotStorage};

const RESTING_ORDER_KEY_SEED: u8 = 2;


pub struct OrderId {
    /// The market index
    pub market_index: u8,

    /// Tick where order is placed
    pub tick: u32,

    /// Resting order index between 0 to 15. A single tick can have at most 15 orders
    pub resting_order_index: u8,
}

impl SlotKey for OrderId {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = RESTING_ORDER_KEY_SEED;
        key[1] = self.market_index;
        key[2..6].copy_from_slice(&self.tick.to_le_bytes());
        key[7] = self.resting_order_index;

        key
    }
}

/// Resting order on a 32 byte slot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlotRestingOrder {
    /// Trader address in big endian. Other fields are in little endian.
    pub trader_address: [u8; 20],
    pub num_base_lots: u32,
    pub last_valid_slot: u32,
    pub last_valid_unix_timestamp_in_seconds: u32,
}

impl SlotRestingOrder {
    pub fn new_default(trader_address: [u8; 20], num_base_lots: u32) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new(
        trader_address: [u8; 20],
        num_base_lots: u32,
        last_valid_slot: Option<u32>,
        last_valid_unix_timestamp_in_seconds: Option<u32>,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_slot: last_valid_slot.unwrap_or(0),
            last_valid_unix_timestamp_in_seconds: last_valid_unix_timestamp_in_seconds.unwrap_or(0),
        }
    }

    pub fn new_with_last_valid_slot(
        trader_address: [u8; 20],
        num_base_lots: u32,
        last_valid_slot: u32,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_slot,
            last_valid_unix_timestamp_in_seconds: 0,
        }
    }

    pub fn new_with_last_valid_unix_timestamp(
        trader_address: [u8; 20],
        num_base_lots: u32,
        last_valid_unix_timestamp_in_seconds: u32,
    ) -> Self {
        SlotRestingOrder {
            trader_address,
            num_base_lots,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds,
        }
    }

    /// Decode CBRestingOrder from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        unsafe { core::mem::transmute::<[u8; 32], SlotRestingOrder>(slot) }
    }

    /// Load CBRestingOrder from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &OrderId) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        SlotRestingOrder::decode(slot)
    }

    /// Encode CBRestingOrder as a 32 bit slot in little endian
    pub fn encode(&self) -> [u8; 32] {
        unsafe { core::mem::transmute::<SlotRestingOrder, [u8; 32]>(*self) }
    }

    /// Encode and save CBRestingOrder to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &OrderId) {
        let encoded = self.encode();

        slot_storage.sstore(&key.get_key(), &encoded);
    }
}

impl RestingOrder for SlotRestingOrder {
    fn size(&self) -> u32 {
        self.num_base_lots
    }

    fn last_valid_slot(&self) -> Option<u32> {
        if self.last_valid_slot == 0 {
            None
        } else {
            Some(self.last_valid_slot)
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
        (self.last_valid_slot != 0 && self.last_valid_slot < current_slot)
            || (self.last_valid_unix_timestamp_in_seconds != 0
                && self.last_valid_unix_timestamp_in_seconds < current_unix_timestamp_in_seconds)
    }
}


#[cfg(test)]
mod test {
    use stylus_sdk::alloy_primitives::Address;

    use super::SlotRestingOrder;

    #[test]
    fn test_encode_resting_order() {
        let resting_order = SlotRestingOrder {
            trader_address: Address::ZERO.0.into(),
            num_base_lots: 1,
            last_valid_slot: 0,
            last_valid_unix_timestamp_in_seconds: 257,
        };

        let encoded_order = resting_order.encode();
        assert_eq!(
            encoded_order,
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 0, 0,
            ]
        );
    }

    #[test]
    fn test_decode_resting_order() {
        let slot: [u8; 32] = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0,
        ];

        let resting_order = SlotRestingOrder::decode(slot);

        // This is 0x0000000000000000000000000000000000000001
        let expected_address =
            Address::from_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(resting_order.trader_address, expected_address);
        assert_eq!(resting_order.num_base_lots, 1);
        assert_eq!(resting_order.last_valid_slot, 0);
        assert_eq!(resting_order.last_valid_unix_timestamp_in_seconds, 257);
    }
}
