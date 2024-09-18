use super::{
    iterator::{
        active_position::active_inner_index_iterator::ActiveInnerIndexIterator,
        position::inner_index_iterator::InnerIndexIterator,
    },
    order::group_position::GroupPosition,
    MarketPrices,
};
use crate::state::{
    slot_storage::{SlotActions, SlotKey, SlotStorage},
    InnerIndex, OuterIndex, RestingOrderIndex, Side,
};

/// A BitmapGroup contains Bitmaps for 32 ticks in ascending order.
/// A single Bitmap contains data of 8 resting orders.
///
/// Bids and Asks have a common set of BitmapGroups because a resting order
/// at a tick can't be on both sides at the same time.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct BitmapGroup {
    pub inner: [u8; 32],
}

impl BitmapGroup {
    pub fn new_from_slot(slot_storage: &SlotStorage, key: OuterIndex) -> Self {
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

    /// Activate bit at the given group position
    pub fn activate(&mut self, group_position: GroupPosition) {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = group_position;

        let mut bitmap = self.get_bitmap_mut(&inner_index);
        bitmap.activate(&resting_order_index);
    }

    /// Activate bit at the given group position
    pub fn deactivate(&mut self, group_position: GroupPosition) {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = group_position;

        let mut bitmap = self.get_bitmap_mut(&inner_index);
        bitmap.clear(&resting_order_index);
    }

    pub fn order_present(&mut self, group_position: GroupPosition) -> bool {
        let GroupPosition {
            inner_index,
            resting_order_index,
        } = group_position;

        let bitmap = self.get_bitmap(&inner_index);
        bitmap.order_present(resting_order_index)
    }

    /// Whether the bitmap group has active resting orders at the given inner index
    pub fn inner_index_is_active(&self, inner_index: InnerIndex) -> bool {
        self.inner[inner_index.as_usize()] != 0
    }

    /// Get the best active inner index in a bitmap group, beginning
    /// from an optional starting position (inclusive)
    ///
    /// Returns None if there is no active index. Externally ensure that this is called on an active
    /// bitmap group.
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `starting_index` - Search beginning from this index (inclusive) if Some,
    /// else begin lookup from the edge of the bitmap group.
    ///
    pub fn best_active_inner_index(
        &self,
        side: Side,
        starting_index: Option<InnerIndex>,
    ) -> Option<InnerIndex> {
        let mut iterator = ActiveInnerIndexIterator::new(self, side, starting_index);
        iterator.next()
    }

    /// Whether the bitmap group is inactive for `side`
    ///
    /// Even if bits for a side have closed, the opposite side bits can remain open.
    /// Therefore avoid `is_active = self.inner != [0u8; 32]`
    ///
    pub fn is_inactive(&self, side: Side, start_index_inclusive: Option<InnerIndex>) -> bool {
        let best_active_index = self.best_active_inner_index(side, start_index_inclusive);
        best_active_index.is_none()
    }

    /// Clear garbage bits in the bitmap group that fall between best market prices
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `outer_index`
    /// * `best_market_price`
    /// * `best_opposite_price`
    ///
    pub fn clear_garbage_bits(
        &mut self,
        outer_index: OuterIndex,
        best_market_prices: &MarketPrices,
    ) {
        let mut iterator =
            InnerIndexIterator::new_between_market_prices(best_market_prices, outer_index);

        while let Some(inner_index_to_clear) = iterator.next() {
            self.inner[inner_index_to_clear.as_usize()] = 0;
        }
    }

    /// Whether the bitmap group is active. If the active state changes then
    /// the tick group list must be updated
    ///
    /// Important- Avoid this function for index list deactivations. Use is_inactive()
    /// instead
    ///
    // pub fn is_active(&self) -> bool {
    //     self.inner != [0u8; 32]
    // }

    /// Write to slot
    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &OuterIndex) {
        slot_storage.sstore(&key.get_key(), &self.inner);
    }

    /// Set a placeholder non-empty value so that the slot is not cleared
    ///
    /// TODO remove. We no longer clear bitmap groups. We simply remove its outer
    /// index from index list
    pub fn set_placeholder(&mut self) {
        self.inner = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bitmap<'a> {
    pub inner: &'a u8,
}

impl Bitmap<'_> {
    pub fn is_empty(&self) -> bool {
        *self.inner == 0
    }

    /// Whether a resting order is present at the given index
    pub fn order_present(&self, index: RestingOrderIndex) -> bool {
        // Use bitwise AND operation to check if the bit at the given index is set
        // If the bit is set, it means that an order is present at that index
        (*self.inner & (1 << index.as_u8())) != 0
    }

    /// Find the best available slot with the lowest index
    pub fn best_free_index(&self, start: u8) -> Option<RestingOrderIndex> {
        // Iterate through each bit starting from the least significant bit
        for i in start..8 {
            let resting_order_index = RestingOrderIndex::new(i);
            // Check if the bit at index `i` is 0
            if !self.order_present(resting_order_index.clone()) {
                return Some(resting_order_index);
            }
        }
        // If all bits are 1, return None indicating no free index
        None
    }

    /// Checks if removing the order at the specified `RestingOrderIndex`
    /// will clear the entire bitmap, meaning no bits will be set after the removal.
    ///
    /// # Arguments
    ///
    /// * `resting_order_index` - The index of the order to be removed.
    ///
    /// # Returns
    ///
    /// * `true` if removing the order will result in the bitmap being cleared (i.e., no bits set).
    /// * `false` if other bits will remain set after the removal.
    pub fn will_be_cleared_after_removal(&self, resting_order_index: RestingOrderIndex) -> bool {
        // Create a mask to clear the bit at `resting_order_index`
        let mask = !(1 << resting_order_index.as_u8());

        // Apply the mask to the bitmap
        // If the result after applying the mask is 0, it means that removing the
        // order will clear the bitmap (all bits will be zero)
        (*self.inner & mask) == 0
    }
}

/// An 8 bit bitmap which tells about active resting orders at the given tick.
// #[derive(Copy, Clone)]
pub struct MutableBitmap<'a> {
    pub inner: &'a mut u8,
}

impl MutableBitmap<'_> {
    pub fn is_empty(&self) -> bool {
        *self.inner == 0
    }

    /// Whether a resting order is present at the given index
    pub fn order_present(&self, index: RestingOrderIndex) -> bool {
        // Use bitwise AND operation to check if the bit at the given index is set
        // If the bit is set, it means that an order is present at that index
        (*self.inner & (1 << index.as_u8())) != 0
    }

    /// Find the best available slot with the lowest index
    pub fn best_free_index(&self, start: u8) -> Option<RestingOrderIndex> {
        // Iterate through each bit starting from the least significant bit
        for i in start..8 {
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

    /// Activate (set to 1) the bit at the given index
    pub fn activate(&mut self, resting_order_index: &RestingOrderIndex) {
        // Use bitwise OR operation to set the bit at the given index
        *self.inner |= 1 << resting_order_index.as_u8();
    }
}

#[cfg(test)]
mod tests {
    use crate::quantities::Ticks;

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

        let bitmap_group = BitmapGroup::new_from_slot(&slot_storage, OuterIndex::new(0));

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

        let bitmap_group = BitmapGroup::new_from_slot(&slot_storage, outer_index);
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

    #[test]
    fn test_is_inactive_for_bids() {
        let mut bitmap_group = BitmapGroup::default();

        let side = Side::Bid;
        let starting_index = InnerIndex::new(10);

        bitmap_group.inner[starting_index.as_usize() + 1] = 0b00000001;
        assert_eq!(bitmap_group.is_inactive(side, Some(starting_index)), true);

        bitmap_group.inner[starting_index.as_usize()] = 0b00000001;
        assert_eq!(bitmap_group.is_inactive(side, Some(starting_index)), false);
    }

    #[test]
    fn test_is_inactive_for_asks() {
        let mut bitmap_group = BitmapGroup::default();

        let side = Side::Ask;
        let starting_index = InnerIndex::new(10);

        bitmap_group.inner[starting_index.as_usize() - 1] = 0b00000001;
        assert_eq!(bitmap_group.is_inactive(side, Some(starting_index)), true);

        bitmap_group.inner[starting_index.as_usize()] = 0b00000001;
        assert_eq!(bitmap_group.is_inactive(side, Some(starting_index)), false);
    }

    // will_be_cleared_after_removal() tests

    #[test]
    fn test_removal_clears_bitmap_single_bit_set() {
        // Bitmap with only the first bit set (0b00000001)
        let inner = 0b00000001;
        let bitmap = Bitmap { inner: &inner };
        let resting_order_index = RestingOrderIndex::new(0);

        // Removing the only order should clear the bitmap
        assert!(bitmap.will_be_cleared_after_removal(resting_order_index));
    }

    #[test]
    fn test_removal_clears_bitmap_middle_bit_set() {
        // Bitmap with only the middle bit set (0b00010000)
        let inner = 0b00010000;
        let bitmap = Bitmap { inner: &inner };
        let resting_order_index = RestingOrderIndex::new(4);

        // Removing the only order should clear the bitmap
        assert!(bitmap.will_be_cleared_after_removal(resting_order_index));
    }

    #[test]
    fn test_removal_does_not_clear_bitmap_other_bits_set() {
        // Bitmap with multiple bits set (0b00000011)
        let inner = 0b00000011;
        let bitmap = Bitmap { inner: &inner };
        let resting_order_index = RestingOrderIndex::new(0);

        // Removing the order at index 0 should NOT clear the bitmap
        // because there is still an order at index 1
        assert!(!bitmap.will_be_cleared_after_removal(resting_order_index));
    }

    #[test]
    fn test_removal_clears_bitmap_highest_bit_set() {
        // Bitmap with highest bit set (0b10000000)
        let inner = 0b10000000;
        let bitmap = Bitmap { inner: &inner };
        let resting_order_index = RestingOrderIndex::new(7);

        // Removing the order at index 7 should clear the bitmap
        assert!(bitmap.will_be_cleared_after_removal(resting_order_index));
    }

    #[test]
    fn test_removal_does_not_clear_bitmap_all_bits_set() {
        // Bitmap with all bits set (0b11111111)
        let inner = 0b11111111;
        let bitmap = Bitmap { inner: &inner };
        let resting_order_index = RestingOrderIndex::new(3);

        // Removing the order at index 3 should not clear the bitmap,
        // since all other bits are still set
        assert!(!bitmap.will_be_cleared_after_removal(resting_order_index));
    }

    #[test]
    fn test_clear_garbage_bits_same_outer_index() {
        let outer_index = OuterIndex::ONE;
        let market_prices = MarketPrices {
            best_bid_price: Ticks::from_indices(outer_index, InnerIndex::new(0)),
            best_ask_price: Ticks::from_indices(outer_index, InnerIndex::new(2)),
        };

        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 1;
        bitmap_group.inner[1] = 1;
        bitmap_group.inner[2] = 1;

        bitmap_group.clear_garbage_bits(outer_index, &market_prices);

        assert_eq!(
            bitmap_group.inner,
            [
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_clear_in_range_best_ask_price_on_different_outer_index() {
        let outer_index = OuterIndex::ONE;
        let market_prices = MarketPrices {
            best_bid_price: Ticks::from_indices(outer_index, InnerIndex::new(0)),
            best_ask_price: Ticks::from_indices(OuterIndex::new(2), InnerIndex::new(2)),
        };

        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 1;
        bitmap_group.inner[1] = 1;
        bitmap_group.inner[2] = 1;

        bitmap_group.clear_garbage_bits(outer_index, &market_prices);

        assert_eq!(
            bitmap_group.inner,
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );
    }
}
