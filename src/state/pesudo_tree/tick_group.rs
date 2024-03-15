use crate::state::slot_storage::{SlotActions, SlotStorage};

const TICK_HEADER_KEY_SEED: u8 = 1;

/// Key to fetch a TickGroup. A TickGroup consists of multiple TickHeaders
pub struct TickGroupKey {
    /// The market index
    pub market_index: u16,

    /// Index of tick header
    pub tick_group_index: u32,
}

impl TickGroupKey {
    pub fn new_from_tick_index(market_index: u16, tick_index: u32) -> Self {
        TickGroupKey {
            market_index,
            // A TickGroup holds 32 TickHeaders
            tick_group_index: tick_index / 32,
        }
    }

    pub fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0..4].copy_from_slice(&self.tick_group_index.to_le_bytes());
        key[4..6].copy_from_slice(&self.market_index.to_le_bytes());
        key[6..7].copy_from_slice(&TICK_HEADER_KEY_SEED.to_le_bytes());

        key
    }
}

/// A decoded TickHeader provides metadata about resting orders at a tick
/// TickHeader is encoded as 8 bits and stored on the TickHeaderSlot
#[derive(Copy, Clone)]
pub struct TickHeader {
    /// Number of resting orders at the tick
    /// Occupies 4 bits for a max value of 15.
    pub order_count: u8,

    /// Head index of the first order in the resting orders circular array
    /// Occupies 4 bits for a max value of 15
    pub head: u8,
}

impl TickHeader {
    pub fn new(tick_header_byte: u8) -> Self {
        let order_count = tick_header_byte & 0x0F; // Extract lower 4 bits
        let head = tick_header_byte >> 4; // Extract upper 4 bits
        TickHeader { order_count, head }
    }

    pub fn encode(&self) -> u8 {
        let encoded_order_count = self.order_count & 0x0F; // Ensure order_count fits into 4 bits
        let encoded_head = self.head << 4; // Shift head to upper 4 bits
        encoded_order_count | encoded_head // Combine order_count and head
    }
}

/// A TickGroup contains 32 contiguous encoded TickHeaders in ascending order.
/// Bids and Asks have a common set of TickGroups because a resting order
/// at a tick can't be on both sides at the same time.
pub struct TickGroup {
    pub inner: [u8; 32],
}

impl TickGroup {
    pub fn new(slot_storage: &SlotStorage, key: &TickGroupKey) -> Self {
        TickGroup {
            inner: slot_storage.sload(&key.get_key()),
        }
    }

    /// Decode all TickHeaders in a TickGroup
    pub fn headers(&self) -> [TickHeader; 32] {
        let mut headers = [TickHeader {
            order_count: 0,
            head: 0,
        }; 32];
        for i in 0..32 {
            headers[i] = TickHeader::new(self.inner[i]);
        }

        headers
    }

    /// Obtain TickHeader at a given index
    /// Must externally ensure that index is less than 32
    pub fn header(&self, header_index: u8) -> TickHeader {
        TickHeader::new(self.inner[header_index as usize])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_header() {
        let tick_header = TickHeader {
            order_count: 1,
            head: 1,
        };

        assert_eq!(tick_header.encode(), 17);
    }

    #[test]
    fn test_decode_header() {
        let tick_header = TickHeader::new(18);

        assert_eq!(tick_header.order_count, 2);
        assert_eq!(tick_header.head, 1);
    }

    #[test]
    fn test_decode_header_from_empty_byte() {
        let tick_header = TickHeader::new(0);

        assert_eq!(tick_header.order_count, 0);
        assert_eq!(tick_header.head, 0);
    }

    #[test]
    fn test_decode_group_from_empty_slot() {
        let slot_storage = SlotStorage::new();

        let tick_group = TickGroup::new(
            &slot_storage,
            &TickGroupKey {
                market_index: 0,
                tick_group_index: 0,
            },
        );

        assert_eq!(tick_group.inner, [0u8; 32]);

        for header in tick_group.headers() {
            assert_eq!(header.order_count, 0);
            assert_eq!(header.head, 0);
        }
    }

    #[test]
    fn test_decode_filled_slot() {
        let mut slot_storage = SlotStorage::new();

        // Tick group 0 contains ticks from 0 to 31
        let key = TickGroupKey {
            market_index: 0,
            tick_group_index: 0,
        };

        let slot_bytes: [u8; 32] = [
            16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        slot_storage.sstore(&key.get_key(), &slot_bytes);

        let tick_header = TickGroup::new(&slot_storage, &key);
        assert_eq!(tick_header.inner, slot_bytes);

        let mut expected_headers = [TickHeader {
            order_count: 0,
            head: 0,
        }; 32];

        expected_headers[0] = TickHeader {
            order_count: 0,
            head: 1,
        };
        expected_headers[1] = TickHeader {
            order_count: 1,
            head: 1,
        };

        let headers = tick_header.headers();

        for i in 0..32 {
            assert_eq!(headers[i].order_count, expected_headers[i].order_count);
            assert_eq!(headers[i].head, expected_headers[i].head);
        }
    }
}
