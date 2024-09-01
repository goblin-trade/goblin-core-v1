An EVM slot consists of 256 bit key and 256 bit value. A list of outer indices is constructed on these slots as follows
- A slot consists of 16 outer indices, each of type u16
- The slot is identified by SlotIndex
- Position of an outer index within a slot is given by RelativeIndex

```rs
pub struct IndexListIterator<'a> {
    pub slot_storage: &'a mut SlotStorage, // Reference to the slot storage
    pub slot_index: u16,
    pub relative_index: u16,
    pub last_element_read: bool,
    pub list_slot: Option<ListSlot>, // Cache the current list_slot
}

impl<'a> IndexListIterator<'a> {
    pub fn new(outer_index_count: u16, slot_storage: &'a mut SlotStorage) -> Self {
        let slot_index = (outer_index_count - 1) / 16;
        let relative_index = (outer_index_count - 1) % 16;

        Self {
            slot_storage,
            slot_index,
            relative_index,
            last_element_read: false, // Initialize with false
            list_slot: None,          // Initialize with None
        }
    }
}

impl<'a> Iterator for IndexListIterator<'a> {
    // Slot index, relative index, list slot for the outer index, and the outer index itself
    type Item = (u16, u16, ListSlot, OuterIndex);

    fn next(&mut self) -> Option<Self::Item> {
```

IndexListIterator iterates over this list of outer indices. This list has following properties
- Outer indices closer to centre of the book are outside, i.e. at a higher absolute index, while indices further away are deeper (lower absolute index)
- The list has no duplicate elements

```rs
#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}
```

Our smart contract has two index lists, one for bids and one for asks. The items in the list are obtained by calling `.next()`.

Write a structure to insert new indices in the index list
- `prepare(outer_index)` function

  - prepares to insert index at a time.
  - This should build an in memory cache, i.e.  a vector of outer indices that will be written to slots when `commit()` is called. The function will read items from `IndexListIterator` and push them into the cache till the correct position for input element is found. The input element is also pushed to the cache.
  - The elements will be in descending order for bids and in ascending order for asks. It is possible that two indices are the same. The duplicate one should be ignored. The function returns true if the `outer_index` was queued, else false if the item was already in the list or if the same element was already inserted. outer indices passed to prepare() are guaranteed to be sorted. No need to sort inside the function.
  - Algorithm
    1. Try to read last element `last_pushed_outer_index` from the cache
    2. If element exists, and is equal to the current index then return false. The outer index already exists in the list.
    3. If `last_pushed_outer_index` is better (closer to the centre) than `outer_index`: this is a special case to handle values returned by next() in a previous prepare() call where the input value was closer to the center. If this condition is true in the current loop then insert `outer_index` before this value in the cache. Return true and exit.
    4. Else read `current_outer_index` from next(). If it doesn't exist, simply write `outer_index` to cache and return true.
    4. If `outer_index` is closer to the centre than `current_outer_index` then push `inner_index` and then `current_outer_index` to cache. Return true.
    5. Else push `current_outer_index` to cache and GOTO 4.


- commit() function to finally write the cache to slot
  - The last value returned by .next() in prepare() gives the inner index, outer index and list item for the last value read from the index list. Use these coordinates when writing the cache to slot.
  - When writing the first slot, the last value of `list item` returned by next can be used (unless the write happens on the next slot). For subsequent slots, create new blank slots using `ListSlot::new()`. This saves gas by using default 0 instead of reading from slot.
  - items in the cache will be written to the index list, slot by slot beginning from outermost element in the cache list.
  - As we insert elements, the current slot index and relative index increase. Wrap around once last relative index is filled.


The goal is to minimize new_from_slot() and write_to_slot() operations.

ListSlot functions are as follows

```rs
//  Read from slot storage
let mut list_slot = ListSlot::new_from_slot(&slot_storage, ListKey { index: 0 });

// Set values
list_slot.inner = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, u16::MAX];
list_slot.set(relative_index, OuterIndex::new(0)); // relative index is of type usize

// Get individual outer index at a given relative index
let current_value = list_slot.get(relative_index);

// Write to slot storage
list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
```

Definitions

- Closer to centre:
  1. Bids: If outer_index > current_outer_index
  2. Asks: If outer_index < current_outer_index
