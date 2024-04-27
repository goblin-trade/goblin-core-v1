use crate::state::slot_storage::{SlotActions, SlotKey, SlotStorage};

const TICK_HEADER_KEY_SEED: u8 = 1;
pub const MAX_ORDERS_PER_TICK: u8 = 15;

/// A Bitmap contains 32 contiguous encoded OrdersAtTick in ascending order.
/// Bids and Asks have a common set of BitmapGroups because a resting order
/// at a tick can't be on both sides at the same time.
pub struct Bitmap {
    pub inner: [u8; 32],
}

impl Bitmap {
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &BitmapKey) -> Self {
        Bitmap {
            inner: slot_storage.sload(&key.get_key()),
        }
    }

    /// Obtain OrdersAtTick at a given index
    /// Must externally ensure that index is less than 32
    pub fn orders(&self, index: usize) -> OrdersAtTick {
        OrdersAtTick {
            inner: self.inner[index],
        }
    }

    /// Update orders at the given index
    pub fn update_orders(&mut self, index: usize, new_orders: &OrdersAtTick) {
        self.inner[index] = new_orders.inner
    }

    /// Whether the bitmap is active. If the active state changes then
    /// the tick group list must be updated
    pub fn is_active(&self) -> bool {
        self.inner != [0u8; 32]
    }

    /// Write to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &BitmapKey) {
        slot_storage.sstore(&key.get_key(), &self.inner);
    }
}

/// Key to fetch a Bitmap. A Bitmap consists of multiple OrdersAtTick
pub struct BitmapKey {
    /// Index of bitmap group
    pub index: u16,
}

impl SlotKey for BitmapKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = TICK_HEADER_KEY_SEED;
        key[1..3].copy_from_slice(&self.index.to_be_bytes());

        key
    }
}

impl BitmapKey {
    /// Obtain bitmap key from tick
    ///
    /// # Arguments
    ///
    /// * `tick` - The price tick of size 2^21. This must be ensured externally.
    ///
    pub fn new_from_tick(tick: u32) -> Self {
        BitmapKey {
            // A Bitmap holds order statuses for 32 ticks
            index: (tick / 32) as u16,
        }
    }
}

/// OrdersAtTick is an 8 bit bitmap which tells
/// about active resting orders at the given tick.
#[derive(Copy, Clone)]
pub struct OrdersAtTick {
    pub inner: u8,
}

impl OrdersAtTick {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_group_from_empty_slot() {
        let slot_storage = SlotStorage::new();

        let bitmap = Bitmap::new_from_slot(&slot_storage, &BitmapKey { index: 0 });

        assert_eq!(bitmap.inner, [0u8; 32]);
    }

    #[test]
    fn test_decode_filled_slot() {
        let mut slot_storage = SlotStorage::new();

        // Tick group 0 contains ticks from 0 to 31
        let key = BitmapKey { index: 0 };

        let slot_bytes: [u8; 32] = [
            16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        slot_storage.sstore(&key.get_key(), &slot_bytes);

        let bitmap = Bitmap::new_from_slot(&slot_storage, &key);
        assert_eq!(bitmap.inner, slot_bytes);

        let orders_at_tick_0 = bitmap.orders(0);
        let orders_at_tick_1 = bitmap.orders(1);

        assert_eq!(orders_at_tick_0.inner, 16);
        assert_eq!(orders_at_tick_1.inner, 17);
        let order_present_expected_0 = [false, false, false, false, true, false, false, false];
        let order_present_expected_1 = [true, false, false, false, true, false, false, false];

        for i in 0..8 {
            assert_eq!(
                orders_at_tick_0.order_present(i),
                order_present_expected_0[i as usize]
            );
            assert_eq!(
                orders_at_tick_1.order_present(i),
                order_present_expected_1[i as usize]
            );
        }
    }
}
