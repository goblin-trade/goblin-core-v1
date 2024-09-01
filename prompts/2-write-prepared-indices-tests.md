```rs
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ListSlot {
    pub inner: [u16; 16],
}

impl ListSlot {
    /// Load from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: ListKey) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        ListSlot::decode(slot)
    }

    /// Decode from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        ListSlot {
            inner: unsafe { core::mem::transmute::<[u8; 32], [u16; 16]>(slot) },
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        unsafe { core::mem::transmute::<[u16; 16], [u8; 32]>(self.inner) }
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &ListKey) {
        let bytes = self.encode();
        slot_storage.sstore(&key.get_key(), &bytes);
    }

    pub fn get(&self, index: usize) -> OuterIndex {
        OuterIndex::new(self.inner[index])
    }

    pub fn set(&mut self, index: usize, value: OuterIndex) {
        self.inner[index] = value.as_u16();
    }
```

A list slot consists of 16 outer indices. Listslots are read from `SlotStorage`, an API to write to disk

```rs
        let mut slot_storage = SlotStorage::new();

        // Setup the initial slot storage with one item
        {
            let mut list_slot = ListSlot::default();
            list_slot.set(0, OuterIndex::new(100));
            list_slot.write_to_slot(&mut slot_storage, &ListKey { index: 0 });
        }
```

The index list is a list of list slots, where each slot can hold 16 outer indices. For example if there are 17 outer indices in the index list, slot 0 holds 16 items and slot 1 holds 1. The position of an item in the list is given by (slot index, relative index).

`write_prepared_indices()` is used to write a list of cached outer indices to the index list.

- Values in the cache are written from the end to start. For example if cache holds [1, 2, 3] then [3, 2, 1] is written.
- `unread_count` is the number of items present in index list, that are not in cache. For example if `unread_count` is 1 and cache size is 2 then total elements after write will be 3.

```rs
pub fn write_prepared_indices(
    slot_storage: &mut SlotStorage,
    cache: &mut Vec<OuterIndex>,
    unread_count: u16,
    first_list_slot: ListSlot,
)
```

Write tests to test this function.
