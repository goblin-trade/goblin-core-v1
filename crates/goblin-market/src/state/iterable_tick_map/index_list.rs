/// Index List is a contiguous list of active outer indices of bitmap groups. A bitmap group
/// is active if at least one of its ticks has a resting order, live or expired.
///
/// There is one list for bids and another for asks. Elements are sorted in descending
/// order for asks and in ascending order for bids.
/// Since ticks in middle of the orderbook are accessed most, having them at the end
/// allows cheaper updates. Updates at beginning of the list, i.e. furthest from middle
/// of the orderbook cost more because the entire list must be shifted right.
///
/// Each tick group index is made of 2 bits in big endian format. This means that
/// each ListItem contains 16 outer indices.
use crate::state::{Side, SlotActions, SlotKey, SlotStorage, LIST_KEY_SEED};
use alloc::vec::Vec;

/// Slot key to fetch a ListSlot
///
/// The number of allowed indices per list is u16::MAX. Since one slot contains 16 indices,
/// the max number of slots are 2^16 / 16 - 1 = 2^12 - 1 = 4095
///
/// Griefing- There will only be enough gas to shift 10-20 slots at a time.
/// What if someone stuffs the slots?
/// Eg. If ETH is at 3700 and tick size is 0.01 USDC. There are 100 ticks between
/// 3700 and 3701, which means 100 / 16 = 6.25 slots.
///
/// Solution- set tick size to 0.1. This way 10 slots equal 10 * 16 * 0.1 = $16 difference
///
/// There's also a game theory element. Ticks near the centre will be filled out eventually.
/// Placing ticks at the end cost more and don't affect placing ticks at the centre.
/// Market makers can be patient if they want to place orders further away from griefing orders.
///
/// Alternative- set a limit on the number of elements in a list. Allow eviction
/// by more aggressive orders. However we are able to add more aggressive orders without
/// extra cost in the current design. Problem arises when somebody wants to place
/// an order further away.
///
pub struct ListKey {
    /// Index of the ListSlot, max 2^12 - 1
    pub index: u16,
}

impl SlotKey for ListKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = LIST_KEY_SEED;
        key[1..3].copy_from_slice(&self.index.to_be_bytes());

        key
    }
}

#[derive(Default)]
pub struct ListSlot {
    pub inner: [u16; 16],
}

impl ListSlot {
    /// Load from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &ListKey) -> Self {
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

    pub fn get(&self, index: usize) -> u16 {
        self.inner[index]
    }

    pub fn set(&mut self, index: usize, value: u16) {
        self.inner[index] = value;
    }
}

/// High level structure for the index list with getter and setter functions
pub struct IndexList {
    /// Bid or ask
    pub side: Side,

    /// Number of active outer indices in the list
    /// The number of slots is given as size / 16 and index of an item is given as size % 16
    pub size: u16,
}

impl IndexList {
    /// Outer indices are sorted in ascending order for bids and in descending order for asks,
    /// such that elements at middle of the orderbook are at the end of the list.
    pub fn ascending(&self) -> bool {
        self.side == Side::Bid
    }

    /// Remove an outer index from the list
    /// Items on the right are left shifted
    ///
    /// TODO remove multiple function?
    pub fn remove(&mut self, slot_storage: &mut SlotStorage, value_to_remove: u16) {
        // Save indices that fall to the right of the removed item in memory.
        // These elements will be left shifted and written to slot.
        let mut read_values = Vec::<u16>::new();

        let mut current_slot = ListSlot::default();

        // Loop through IndexList slot from behind
        let mut i = self.size;
        while i > 0 {
            i -= 1;

            let slot_index = i / 16;
            let relative_index = i as usize % 16;

            // Read and decode list slot if this is the first time, or we have entered a new slot
            // If index = 15, the previous slot is exhausted. Need to load a new one.
            if i == self.size - 1 || relative_index == 15 {
                let key = ListKey { index: slot_index };

                current_slot = ListSlot::new_from_slot(slot_storage, &key);
            }
            let current_value = current_slot.get(relative_index);

            if current_value == value_to_remove {
                // item to remove found
                break;
            } else {
                read_values.push(current_value);
            }
        }
        // update current slot, and all slots to the right
        for j in 0..read_values.len() {
            let absolute_index = i as usize + j;
            let slot_index = absolute_index / 16;
            let relative_index = absolute_index % 16;

            current_slot.set(relative_index, read_values[j]);

            // Slot fully populated or list exhausted
            if relative_index == 15 || j == read_values.len() - 1 {
                // Write
                let key = ListKey {
                    index: slot_index as u16,
                };
                current_slot.write_to_slot(slot_storage, &key);

                if j != read_values.len() - 1 {
                    // Prepare empty slot
                    current_slot.inner = [0u16; 16];
                }
            }
        }

        self.size -= 1;
    }

    /// Insert an index into the list
    ///
    /// We iterate from the end of the list to find the right index to insert at
    ///
    /// Since the list holds 16 outer indices per slot, we need to rewrite
    /// the slot into which the inserted item falls, and also rewrite elements on the
    /// right to account for a right shift.
    ///
    /// We must externally ensure that the inserted item is not already present.
    ///
    pub fn insert(&mut self, slot_storage: &mut SlotStorage, new_value: u16) {
        // Save indices that fall to the right of the inserted item in memory.
        // These elements will be right shifted and written to slot.
        let mut read_values = Vec::<u16>::new();

        let mut current_slot = ListSlot::default();

        // Loop through IndexList slot from behind
        let mut i = self.size;
        while i > 0 {
            i -= 1;

            let slot_index = i / 16;
            let relative_index = i % 16;

            // Fetch slot if this is the first time, or we have exhausted the slot's items
            if i == self.size - 1 || relative_index == 15 {
                let key = ListKey { index: slot_index };

                current_slot = ListSlot::new_from_slot(slot_storage, &key);
            }

            let current_value = current_slot.inner[relative_index as usize];

            // check whether the new value is to be inserted after the current one
            if (self.ascending() && new_value < current_value)
                || (!self.ascending() && new_value > current_value)
            {
                read_values.push(current_value);
            } else {
                i += 1;
                break;
            }
        }

        // relative index where new value will be added
        let relative_index = i % 16;

        // push the element to insert at top of the stack
        read_values.push(new_value);

        let mut list_slot = ListSlot::default();

        // save elements on left of new_group
        let values_on_left = &current_slot.inner[0..(relative_index as usize)];
        list_slot.inner[0..(relative_index as usize)].copy_from_slice(values_on_left);

        // right shift and write slot
        for j in 0..read_values.len() {
            let absolute_index = i as usize + j;
            let slot_index = absolute_index / 16;
            let relative_index = absolute_index % 16;

            // pop group from stack and add to the slot
            list_slot.inner[relative_index] = read_values.pop().unwrap();

            // If the last element of the slot was entered or the list is exhausted, write and flush the slot
            if relative_index == 15 || read_values.is_empty() {
                let key = ListKey {
                    index: slot_index as u16,
                };

                list_slot.write_to_slot(slot_storage, &key);

                // reset to empty slot
                list_slot = ListSlot::default();
            }
        }

        self.size += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let bytes: [u8; 32] = [
            0, 0, 1, 0, 2, 0, 3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let list_slot = ListSlot::decode(bytes);
        assert_eq!(
            list_slot.inner,
            [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_decode() {
        let list_slot = ListSlot {
            inner: [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        assert_eq!(
            list_slot.encode(),
            [
                0, 0, 1, 0, 2, 0, 3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_insert_for_bids() {
        let mut slot_storage = SlotStorage::new();

        let side = Side::Bid;

        let mut list = IndexList { side, size: 0 };

        // 1. insert first item
        list.insert(&mut slot_storage, 1);

        assert_eq!(list.size, 1);
        let key = ListKey { index: 0 };

        let mut list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert second item
        list.insert(&mut slot_storage, 2);

        assert_eq!(list.size, 2);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert third item
        list.insert(&mut slot_storage, 4);

        assert_eq!(list.size, 3);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [1, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert forth item in the middle
        list.insert(&mut slot_storage, 3);

        assert_eq!(list.size, 4);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_insert_for_asks() {
        let mut slot_storage = SlotStorage::new();

        let side = Side::Ask;

        let mut list = IndexList { side, size: 0 };

        // 1. insert first item
        list.insert(&mut slot_storage, 4);

        assert_eq!(list.size, 1);
        let key = ListKey { index: 0 };

        let mut list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert second item
        list.insert(&mut slot_storage, 2);

        assert_eq!(list.size, 2);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert third item
        list.insert(&mut slot_storage, 1);

        assert_eq!(list.size, 3);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert forth item in the middle
        list.insert(&mut slot_storage, 3);

        assert_eq!(list.size, 4);

        list_slot = ListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            list_slot.inner,
            [4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_insert_across_multiple_slots_for_bids() {
        let mut slot_storage = SlotStorage::new();

        let side = Side::Bid;

        let mut list = IndexList { side, size: 16 };

        let key_0 = ListKey { index: 0 };
        let key_1 = ListKey { index: 1 };

        let initial_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17];

        let slot_0_initial = ListSlot {
            inner: initial_values,
        }
        .encode();

        slot_storage.sstore(&key_0.get_key(), &slot_0_initial);

        // 1. insert element at end
        list.insert(&mut slot_storage, 18);
        assert_eq!(list.size, 17);

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, &key_0);
        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            list_slot_0.inner,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17,]
        );

        assert_eq!(
            list_slot_1.inner,
            [18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert element in middle
        list.insert(&mut slot_storage, 16);
        assert_eq!(list.size, 18);

        list_slot_0 = ListSlot::new_from_slot(&slot_storage, &key_0);
        list_slot_1 = ListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            list_slot_0.inner,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,]
        );

        assert_eq!(
            list_slot_1.inner,
            [17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_insert_across_multiple_slots_for_asks() {
        let mut slot_storage = SlotStorage::new();

        let side = Side::Ask;

        let mut list = IndexList { side, size: 16 };

        let key_0 = ListKey { index: 0 };
        let key_1 = ListKey { index: 1 };

        let initial_values = [18, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

        let slot_0_initial = ListSlot {
            inner: initial_values,
        }
        .encode();

        slot_storage.sstore(&key_0.get_key(), &slot_0_initial);

        // 1. insert element at end
        list.insert(&mut slot_storage, 1);
        assert_eq!(list.size, 17);

        let mut list_slot_0 = ListSlot::new_from_slot(&slot_storage, &key_0);
        let mut list_slot_1 = ListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            list_slot_0.inner,
            [18, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2,]
        );

        assert_eq!(
            list_slot_1.inner,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert element in middle
        list.insert(&mut slot_storage, 17);
        assert_eq!(list.size, 18);

        list_slot_0 = ListSlot::new_from_slot(&slot_storage, &key_0);
        list_slot_1 = ListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            list_slot_0.inner,
            [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3,]
        );

        assert_eq!(
            list_slot_1.inner,
            [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }
}
