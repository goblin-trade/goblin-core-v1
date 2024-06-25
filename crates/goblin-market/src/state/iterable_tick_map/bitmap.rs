use crate::state::{
    slot_storage::{SlotActions, SlotKey, SlotStorage},
    InnerIndex, OuterIndex, RestingOrderIndex, Side,
};
use alloc::boxed::Box;
use core::ops::{Range, RangeInclusive};

/// A BitmapGroup contains Bitmaps for 32 ticks in ascending order.
/// A single Bitmap contains data of 8 resting orders.
///
/// Bids and Asks have a common set of BitmapGroups because a resting order
/// at a tick can't be on both sides at the same time.
#[derive(Clone, Copy)]
pub struct BitmapGroup {
    pub inner: [u8; 32],
}

impl BitmapGroup {
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &OuterIndex) -> Self {
        BitmapGroup {
            inner: slot_storage.sload(&key.get_key()),
        }
    }

    /// Obtain Bitmap at a given index
    pub fn get_bitmap(&self, inner_index: &InnerIndex) -> Bitmap {
        Bitmap {
            inner: &self.inner[inner_index.as_usize()],
        }
    }

    pub fn get_bitmap_mut(&mut self, inner_index: &InnerIndex) -> MutableBitmap {
        MutableBitmap {
            inner: &mut self.inner[inner_index.as_usize()],
        }
    }

    /// Whether the bitmap group is active. If the active state changes then
    /// the tick group list must be updated
    pub fn is_active(&self) -> bool {
        self.inner != [0u8; 32]
    }

    /// Write to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &OuterIndex) {
        slot_storage.sstore(&key.get_key(), &self.inner);
    }

    /// Set a placeholder non-empty value so that the slot is not cleared
    pub fn set_placeholder(&mut self) {
        self.inner = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
    }

    /// Return an iterator to traverse bitmaps in a bitmap group
    ///
    /// Traversal is away from the centre. Move from top to bottom for bids (high to low) and from
    /// bottom to top (low to high) for asks.
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `previous_inner_index` - Begin traversal starting with this index (inclusive)
    ///
    pub fn bitmap_iterator(
        &self,
        side: Side,
        previous_inner_index: Option<InnerIndex>,
    ) -> Box<dyn Iterator<Item = usize>> {
        match side {
            // Top to bottom for bids
            Side::Bid => {
                let highest_index = previous_inner_index.map(|i| i.as_usize()).unwrap_or(31);
                return Box::new((0..=highest_index).rev());
            }
            // Bottom to top for asks
            Side::Ask => {
                let lowest_index = previous_inner_index.map(|i| i.as_usize()).unwrap_or(0);
                return Box::new(lowest_index..=31);
            }
        }
    }

    /// Get the best active inner index in a bitmap group.
    ///
    /// Returns None if there is no active index. Externally ensure that this is called on an active
    /// bitmap group.
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `previous_inner_index` - The previous best inner index, optional. If no value is
    /// provided search from the end. If provided, search beginning from this index (inclusive).
    ///
    pub fn best_active_index(
        &self,
        side: Side,
        previous_inner_index: Option<InnerIndex>,
    ) -> Option<InnerIndex> {
        for i in self.bitmap_iterator(side, previous_inner_index) {
            if self.inner[i] != 0 {
                return Some(InnerIndex::new(i));
            }
        }

        None
    }
}

pub struct BitmapGroupWithIndex {
    pub bitmap_group: BitmapGroup,
    pub outer_index: OuterIndex,
}

pub struct Bitmap<'a> {
    pub inner: &'a u8,
}

impl Bitmap<'_> {
    pub fn empty(&self) -> bool {
        *self.inner == 0
    }

    /// Whether a resting order is present at the given index
    pub fn order_present(&self, index: RestingOrderIndex) -> bool {
        // Use bitwise AND operation to check if the bit at the given index is set
        // If the bit is set, it means that an order is present at that index
        (*self.inner & (1 << index.as_u8())) != 0
    }

    /// Find the best available slot with the lowest index
    pub fn best_free_index(&self) -> Option<RestingOrderIndex> {
        // Iterate through each bit starting from the least significant bit
        for i in 0..8 {
            let resting_order_index = RestingOrderIndex::new(i);
            // Check if the bit at index `i` is 0
            if !self.order_present(resting_order_index.clone()) {
                return Some(resting_order_index);
            }
        }
        // If all bits are 1, return None indicating no free index
        None
    }
}

/// An 8 bit bitmap which tells about active resting orders at the given tick.
// #[derive(Copy, Clone)]
pub struct MutableBitmap<'a> {
    pub inner: &'a mut u8,
}

impl MutableBitmap<'_> {
    pub fn empty(&self) -> bool {
        *self.inner == 0
    }

    /// Whether a resting order is present at the given index
    pub fn order_present(&self, index: RestingOrderIndex) -> bool {
        // Use bitwise AND operation to check if the bit at the given index is set
        // If the bit is set, it means that an order is present at that index
        (*self.inner & (1 << index.as_u8())) != 0
    }

    /// Find the best available slot with the lowest index
    pub fn best_free_index(&self) -> Option<RestingOrderIndex> {
        // Iterate through each bit starting from the least significant bit
        for i in 0..8 {
            let resting_order_index = RestingOrderIndex::new(i);
            // Check if the bit at index `i` is 0
            if !self.order_present(resting_order_index.clone()) {
                return Some(resting_order_index);
            }
        }
        // If all bits are 1, return None indicating no free index
        None
    }

    /// Flip the bit at the given index
    pub fn flip(&mut self, resting_order_index: &RestingOrderIndex) {
        // Use bitwise XOR operation with a mask to flip the bit at the given index
        *self.inner ^= 1 << resting_order_index.as_u8();
    }

    /// Clear the bit at the given index
    pub fn clear(&mut self, resting_order_index: &RestingOrderIndex) {
        // Use bitwise AND operation with 0 at the given index to clear the bit
        *self.inner &= !(1 << resting_order_index.as_u8());
    }
}

#[cfg(test)]
mod test {
    use crate::state::OuterIndex;

    use super::*;

    #[test]
    fn test_clear() {
        let mut value = 0b0100_0001;
        let mut bitmap = MutableBitmap { inner: &mut value };

        bitmap.clear(&RestingOrderIndex::new(6));

        assert_eq!(value, 0b0000_0001);
    }

    #[test]
    fn test_flip() {
        let mut value = 0b0000_0001;
        let mut bitmap = MutableBitmap { inner: &mut value };

        bitmap.flip(&RestingOrderIndex::new(6));

        assert_eq!(value, 0b0100_0001);
    }

    #[test]
    fn test_decode_group_from_empty_slot() {
        let slot_storage = SlotStorage::new();

        let bitmap_group = BitmapGroup::new_from_slot(&slot_storage, &OuterIndex::new(0));

        assert_eq!(bitmap_group.inner, [0u8; 32]);
    }

    #[test]
    fn test_decode_filled_slot() {
        let mut slot_storage = SlotStorage::new();

        // Tick group 0 contains ticks from 0 to 31
        let outer_index = OuterIndex::new(0);

        let slot_bytes: [u8; 32] = [
            16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        slot_storage.sstore(&outer_index.get_key(), &slot_bytes);

        let bitmap_group = BitmapGroup::new_from_slot(&slot_storage, &outer_index);
        assert_eq!(bitmap_group.inner, slot_bytes);

        let bitmap_0 = bitmap_group.get_bitmap(&InnerIndex::new(0));
        let bitmap_1 = bitmap_group.get_bitmap(&InnerIndex::new(1));

        assert_eq!(*bitmap_0.inner, 16);
        assert_eq!(*bitmap_1.inner, 17);
        let order_present_expected_0 = [false, false, false, false, true, false, false, false];
        let order_present_expected_1 = [true, false, false, false, true, false, false, false];

        for i in 0..8 {
            let resting_order_index = RestingOrderIndex::new(i);

            assert_eq!(
                bitmap_0.order_present(resting_order_index.clone()),
                order_present_expected_0[i as usize]
            );
            assert_eq!(
                bitmap_1.order_present(resting_order_index.clone()),
                order_present_expected_1[i as usize]
            );
        }
    }

    #[test]
    fn bitmap_updates_affect_the_bitmap_group() {
        let mut bitmap_group = BitmapGroup { inner: [0u8; 32] };

        let mut bitmap = bitmap_group.get_bitmap_mut(&InnerIndex::new(0));
        bitmap.flip(&RestingOrderIndex::new(0));

        assert_eq!(*bitmap.inner, 1);
        assert_eq!(bitmap_group.inner[0], 1);
    }
}
