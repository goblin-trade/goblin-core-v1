The following code inserts outer indices in the index list and commits them

```rs
/// Enables bulk insertion of outer indices in the index list.
/// Successive inserted orders should move away from the centre, i.e.
///
/// - insert bids in descending order
/// - insert asks in ascending order
///
pub struct IndexListInserter {
    /// Iterator to read saved values from list
    pub index_list_reader: IndexListReader,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be inserted and values popped from index list reader
    /// in the correct order of insertion.
    pub cache: Vec<OuterIndex>,
}

impl IndexListInserter {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            index_list_reader: IndexListReader::new(outer_index_count, side),
            cache: Vec::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_reader.side
    }

    /// Prepare an outer index for insertion in the index list
    ///
    /// # Arguments
    ///
    /// * outer_index
    /// * slot_storage
    ///
    pub fn prepare(&mut self, slot_storage: &SlotStorage, outer_index: OuterIndex) -> bool {
        // Check last element in the cache
        if let Some(&last_pushed_outer_index) = self.cache.last() {
            // If the element already exists in the cache, return false
            if last_pushed_outer_index == outer_index {
                return false;
            }

            // If the last element in cache is closer to the center than outer_index, insert before it
            if last_pushed_outer_index.is_closer_to_center(self.side(), outer_index) {
                self.cache.pop(); // Remove the last pushed index
                self.cache.push(outer_index);
                self.cache.push(last_pushed_outer_index); // Push it back after the new index
                return true;
            }
        }

        // Iterate through the list to find the correct position
        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_reader.next(slot_storage)
        {
            // If the outer_index is already in the list, only insert once
            if current_outer_index == outer_index {
                self.cache.push(current_outer_index);
                return false;
            }

            // If outer_index is closer to the center, insert before current_outer_index
            if current_outer_index.is_closer_to_center(self.side(), outer_index) {
                self.cache.push(outer_index);
                self.cache.push(current_outer_index);
                return true;
            } else {
                // Otherwise, push the current_outer_index to cache and continue
                self.cache.push(current_outer_index);
            }
        }

        // If reached end without finding a suitable position, push the outer_index to cache
        self.cache.push(outer_index);
        true
    }

    /// Write prepared indices to slot
    pub fn write_prepared_indices(&mut self, slot_storage: &mut SlotStorage) {
        write_prepared_indices(
            slot_storage,
            self.side(),
            &mut self.cache,
            self.index_list_reader.outer_index_count,
            self.index_list_reader.list_slot,
        );
    }
}
```

Here IndexList is initialized like this

```rs
/// Read outer indices from the index list, end first.
/// In an index list, indices closer to the centre are at the end while
/// indices that are away are at the beginning of the list.
pub struct IndexListReader {
    /// Whether bid or ask. There are two lists, one for bids and one for asks.
    pub side: Side,

    /// Number of indices yet to be read
    pub outer_index_count: u16,

    /// The currently read list slot
    pub list_slot: Option<ListSlot>,
}

impl IndexListReader {
    pub fn new(outer_index_count: u16, side: Side) -> Self {
        Self {
            outer_index_count,
            list_slot: None, // Initialize with None
            side,
        }
    }
```

Rules
- Slot storage is in the form of a 256 bit key and 256 bit value. Slot storage is expensive so optimize to reduce write_to_slot() operations.
- A list_slot is made of 16 outer indices where OuterIndex.inner is of type u16.
- Outer indices are arranged in ascending order for bids and descending for asks, i.e. indices closer to the centre of the book are at the end of the list.
- If all values in a slot become 0, do not clear the slot by writing 0s to save gas. Write [u16::MAX, 16]

Write an IndexListRemover
- remove(outer_index) will prepare the cache with the given value removed. When a value is removed, values on the right are left shifted to fill the space.
- write_prepared_indices will write the cache to slot. The writing logic is identical to IndexListInserter that is we build a cache and use coordinates to write values to slot.
- Use index_list_reader.next() to read outer indices from the index list.

Example
- Initially a bids index list has values [1, 2, 3]
- After calling remove(2) and writing, the new list should be [1, 3]. Values on the right should be left shifted.

Example with cleared slot
- Initially the list had 16 non zero values.
- All the values were cleared.
- The slot should be [u16::MAX; 16]
