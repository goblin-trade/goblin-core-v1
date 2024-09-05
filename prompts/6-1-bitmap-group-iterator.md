```rs
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
}

#[derive(Clone, Copy)]
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
}
```

- A bitmap group is a 256 bit struct holding 32 bitmaps of 8 bits each.
- A bitmap represents an inner index. A bitmap group represents an outer index. Together these two indices form a tick price.
- A single tick can have upto 8 resting orders. A resting order is present at a tick if the bit at `resting_order_index` is active.
- The coordinates of an order are given by (outer_index, inner_index, resting_order_index)

The inner structs are defined thus. Their instances are created using `::new()` functions

```rs
/// Key to fetch a Bitmap group. A Bitmap consists of multiple Bitmaps
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct OuterIndex {
    /// Index of bitmap group
    pub inner: u16,
}

/// Key to fetch the bitmap within a bitmap group
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct InnerIndex {
    /// Relative position of the bitmap within the bitmap group
    inner: usize,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct RestingOrderIndex {
    inner: u8,
}
```

Implement an iterable to find the next active bit in a bitmap group. The function should give (inner index, resting order index)
- The iterator struct should hold `side: Side`.
- For bids: move from highest inner index (31) to lowest (0)
- For asks: move from lowest inner index (0) to highest (31)
- The constructor function takes a field `starting_position_to_exclude: Option<InnerIndex, RestingOrderIndex>`. We should start lookup after this position.
- Example: If starting_position_to_exclude = (0, 0) for asks then start lookup from (0, 1).
- Use this variable to index while looping.

```rs
pub struct BitmapGroupIterator<'a> {
    bitmap_group: &'a BitmapGroup,
    side: Side,
    current_index: Option<(InnerIndex, RestingOrderIndex)>,
}
```
