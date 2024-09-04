```rs
/// Read outer indices from the index list, end first.
/// In an index list, indices closer to the centre are at the end while
/// indices that are away are at the beginning of the list. That is
///
/// - bids are in ascending order
/// - asks are in descending order
///
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

    /// Read the next outer index
    ///
    /// # Arguments
    ///
    /// * slot_storage
    ///
    /// # Returns
    ///
    /// The coordinates (slot index, relative index, list_slot) and value of the outer index
    ///
    pub fn next(&mut self, slot_storage: &SlotStorage) -> Option<(u16, u16, ListSlot, OuterIndex)> {
        if self.outer_index_count == 0 {
            return None; // End iteration if no elements left
        }

        // Calculate slot index and relative index
        let slot_index = (self.outer_index_count - 1) / 16;
        let relative_index = (self.outer_index_count - 1) % 16;

        // Check if we need to load a new list_slot
        if self.list_slot.is_none() || relative_index == 15 {
            let list_key = ListKey {
                index: slot_index,
                side: self.side,
            };
            self.list_slot = Some(ListSlot::new_from_slot(slot_storage, list_key));
        }

        // Safe to unwrap because we just initialized it if it was None
        let list_slot = self.list_slot.as_ref().unwrap();

        // Read the outer index from the list slot
        let current_outer_index = list_slot.get(relative_index as usize);

        // Prepare the result
        let result = (slot_index, relative_index, *list_slot, current_outer_index);

        // Decrement the outer_index_count for the next iteration
        self.outer_index_count -= 1;

        Some(result)
    }
}
```

```rs
use crate::state::{OuterIndex, Side, SlotStorage};
use alloc::vec::Vec;

use super::{write_prepared_indices, IndexListReader};

/// Enables bulk removal of outer indices from the index list.
/// Successive removed orders should be away from the center, i.e.,
///
/// - remove bids in ascending order
/// - remove asks in descending order
///
/// Removal can still leave ghost values in the index list. Use
/// MarketState::outer_index_count() to find the correct starting position.
/// Ghost values prevent slots from clearing thus saving gas.
///
/// Ghost values are produced when
///
/// 1. A slot was supposed to close. Instead values in the slot remain.
///
/// 2. Values are removed from the outermost slot but all values are not cleared.
/// Ghost values are because of the cached list slot. Values on right of the
/// removed values are copied into the space, but they are not cleared.
///
pub struct IndexListRemover {
    /// Iterator to read saved values from list
    pub index_list_reader: IndexListReader,

    /// List of cached outer indices which will be written back to slots.
    /// Contains values to be retained after removal.
    pub cache: Vec<OuterIndex>,
}

impl IndexListRemover {
    pub fn new(side: Side, outer_index_count: u16) -> Self {
        Self {
            index_list_reader: IndexListReader::new(outer_index_count, side),
            cache: Vec::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.index_list_reader.side
    }

    /// Prepare the index list by removing the specified outer index
    ///
    /// # Arguments
    ///
    /// * outer_index - The index to be removed
    /// * slot_storage - The slot storage to read indices from
    ///
    pub fn remove(&mut self, slot_storage: &SlotStorage, outer_index: OuterIndex) -> bool {
        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_reader.next(slot_storage)
        {
            // Skip the outer index if it matches the one to be removed
            if current_outer_index == outer_index {
                return true;
            }
            // Push other indices to the cache
            self.cache.push(current_outer_index);
        }

        false
    }

    /// Write prepared indices to slot after removal
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


- A storage slot is made up of a 256 bit key and 256 bit value.
- We have constructed an index list structure which stores 16 bit OuterIndices.
- IndexListReader is used to read these values beginning from the end of the list. For example if the list contains [1, 2, 3] then .next() will return 3, 2 and finally 1.
- Index list elements are sorted. If the side is bids, they are sorted in ascending order and for asks in descending order. The sorting order is decided externally, however the elements are guaranteed to be sorted. There are no duplicate values.
- slot_storage operations that read or write to slot are expensive.

IndexListRemover wraps IndexListReader and performs removals from the index list. It uses 2 functions for efficient batch removals.
- remove() removes one index at a time. A cache is prepared
- write_prepared_indices() will write the cache such that we have a updated index list with given values removed.

Write a new function find_outer_index() which tells whether a given value is present in the list. There are two possibilities for a looked up index
- Remove this index: remove() should be efficient and should not lead to more iterations. This could be tracked either by adding a new variable to the struct, or by temporarily pushing it on the stack.
- Do nothing. Instead we try to find or remove another index. In that case the index should be present in the cache so it is written to slot later.
