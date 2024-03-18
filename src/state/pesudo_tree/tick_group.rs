use crate::state::slot_storage::{SlotActions, SlotKey, SlotStorage};

const TICK_HEADER_KEY_SEED: u8 = 1;
pub const MAX_ORDERS_PER_TICK: u8 = 15;

/// A TickGroup contains 32 contiguous encoded TickHeaders in ascending order.
/// Bids and Asks have a common set of TickGroups because a resting order
/// at a tick can't be on both sides at the same time.
pub struct TickGroup {
    pub inner: [u8; 32],
}

impl TickGroup {
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &TickGroupKey) -> Self {
        TickGroup {
            inner: slot_storage.sload(&key.get_key()),
        }
    }

    /// Obtain TickBitmap at a given index
    /// Must externally ensure that index is less than 32
    pub fn bitmap(&self, index: usize) -> TickBitmap {
        TickBitmap {
            inner: self.inner[index],
        }
    }

    /// Update bitmap at the given index
    pub fn update_bitmap(&mut self, index: usize, new_bitmap: &TickBitmap) {
        self.inner[index] = new_bitmap.inner
    }

    /// Whether the tick group is active. If the active state changes then
    /// the tick group list must be updated
    pub fn is_active(&self) -> bool {
        self.inner != [0u8; 32]
    }

    /// Write Tick group to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &TickGroupKey) {
        slot_storage.sstore(&key.get_key(), &self.inner);
    }
}

/// TickBitmap is an 8 bit bitmap in little endian format which tells
/// about active resting orders at the given tick.
#[derive(Copy, Clone)]
pub struct TickBitmap {
    pub inner: u8,
}

impl TickBitmap {
    /// Whether a resting order is present at the given index
    /// Must externally ensure that `index` is less than 8
    pub fn order_present(&self, index: u8) -> bool {
        // Use bitwise AND operation to check if the bit at the given index is set
        // If the bit is set, it means that an order is present at that index
        (self.inner & (1 << index)) != 0
    }

    /// Find the best available slot with the lowest index
    pub fn best_free_index(&self) -> Option<u8> {
        // Iterate through each bit starting from the least significant bit
        for i in 0..8 {
            // Check if the bit at index `i` is 0
            if !self.order_present(i) {
                return Some(i);
            }
        }
        // If all bits are 1, return None indicating no free index
        None
    }

    /// Flip the bit at the given index
    /// Must externally ensure that `index` is less than 8
    pub fn flip(&mut self, index: u8) {
        // Use bitwise XOR operation with a mask to flip the bit at the given index
        self.inner ^= 1 << index;
    }
}

/// Key to fetch a TickGroup. A TickGroup consists of multiple TickHeaders
pub struct TickGroupKey {
    /// The market index
    pub market_index: u8,

    /// Index of tick header
    pub index: u16,
}

impl SlotKey for TickGroupKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = TICK_HEADER_KEY_SEED;
        key[1] = self.market_index;
        key[2..4].copy_from_slice(&self.index.to_le_bytes());

        key
    }
}

impl TickGroupKey {
    /// Obtain tick group key from a tick
    ///
    /// # Arguments
    ///
    /// * `market_index` - The market index
    /// * `tick` - The price tick of size 2^21. This must be ensured externally.
    ///
    pub fn new_from_tick(market_index: u8, tick: u32) -> Self {
        TickGroupKey {
            market_index,
            // A TickGroup holds headers for 32 ticks
            index: (tick / 32) as u16,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_group_from_empty_slot() {
        let slot_storage = SlotStorage::new();

        let tick_group = TickGroup::new_from_slot(
            &slot_storage,
            &TickGroupKey {
                market_index: 0,
                index: 0,
            },
        );

        assert_eq!(tick_group.inner, [0u8; 32]);
    }

    #[test]
    fn test_decode_filled_slot() {
        let mut slot_storage = SlotStorage::new();

        // Tick group 0 contains ticks from 0 to 31
        let key = TickGroupKey {
            market_index: 0,
            index: 0,
        };

        let slot_bytes: [u8; 32] = [
            16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        slot_storage.sstore(&key.get_key(), &slot_bytes);

        let tick_header = TickGroup::new_from_slot(&slot_storage, &key);
        assert_eq!(tick_header.inner, slot_bytes);

        let bitmap_0 = tick_header.bitmap(0);
        let bitmap_1 = tick_header.bitmap(1);

        assert_eq!(bitmap_0.inner, 16);
        assert_eq!(bitmap_1.inner, 17);
        let order_present_expected_0 = [false, false, false, false, true, false, false, false];
        let order_present_expected_1 = [true, false, false, false, true, false, false, false];

        for i in 0..8 {
            assert_eq!(
                bitmap_0.order_present(i),
                order_present_expected_0[i as usize]
            );
            assert_eq!(
                bitmap_1.order_present(i),
                order_present_expected_1[i as usize]
            );
        }
    }
}
