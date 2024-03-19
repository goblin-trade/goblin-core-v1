use crate::state::{Side, SlotActions, SlotKey, SlotStorage};
use alloc::vec::Vec;

const TICK_GROUP_LIST_KEY_SEED: u8 = 0;

/// A contiguous list of active bitmaps. A bitmap is active
/// if at least one of its ticks has a resting order, live or expired.
///
/// The elements are sorted in descending order for asks and in ascending order for bids.
/// Since ticks in middle of the orderbook are accessed most, having them at the end
/// allows cheaper updates. Updates at beginning of the list, i.e. furthest from middle
/// of the orderbook cost more because the entire list must be shifted right.
///
/// Each tick group index is made of 2 bits in little endian format. This means that
/// each ActiveBitmapsItem contains 16 tick group indices.
///
/// TODO handle insertion deletion inside pseudo_tree
pub struct BitmapList {
    /// The market index
    pub market_index: u8,

    /// Bid or ask
    pub side: Side,

    /// Number of active bitmaps
    pub size: u16,
}

impl BitmapList {
    /// Bitmap indices are sorted in ascending order for bids and in descending order for asks,
    /// such that elements at middle of the orderbook are at the end of the list.
    pub fn ascending(&self) -> bool {
        self.side == Side::Bid
    }

    /// Insert a bitmap into the BitmapList
    ///
    /// We iterate from the end of the list to find the right index to insert at
    ///
    /// Since the list holds 16 bitmap indices per slot, we need to rewrite
    /// the slot into which the inserted item falls, and also rewrite elements on the
    /// right to account for a right shift.
    ///
    /// We must externally ensure that the inserted item is not already present.
    ///
    pub fn insert(&mut self, slot_storage: &mut SlotStorage, new_bitmap_index: u16) {
        // Save read bitmap indices that fall to the right of the inserted item in memory.
        // These elements will be right shifted and written to slot.
        let mut read_bitmaps = Vec::<u16>::new();

        let mut current_slot = BitmapListSlot::default();

        // Loop through BitmapList slot from behind
        let mut i = self.size;
        while i > 0 {
            i -= 1;

            let slot_index = i / 16;
            let relative_index = i % 16;

            // Fetch slot if this is the first time, or we have exhausted the slot's items
            if i == self.size - 1 || relative_index == 15 {
                let key = BitmapListSlotKey {
                    market_index: self.market_index,
                    index: slot_index,
                };

                current_slot = BitmapListSlot::new_from_slot(slot_storage, &key);
            }

            let current_bitmap_index = current_slot.inner[relative_index as usize];

            // check whether the new bitmap index is to be inserted after the current one
            if (self.ascending() && new_bitmap_index < current_bitmap_index)
                || (!self.ascending() && new_bitmap_index > current_bitmap_index)
            {
                read_bitmaps.push(current_bitmap_index);
            } else {
                i += 1;
                break;
            }
        }

        // relative index where new_bitmap_index will be added
        let relative_index = i % 16;

        // push the element to insert at top of the stack
        read_bitmaps.push(new_bitmap_index);

        let mut bitmap_list = BitmapListSlot::default();

        // save elements on left of new_group
        let bitmaps_on_left = &current_slot.inner[0..(relative_index as usize)];
        bitmap_list.inner[0..(relative_index as usize)].copy_from_slice(bitmaps_on_left);

        // right shift and write slot
        for j in 0..read_bitmaps.len() {
            let absolute_index = i as usize + j;
            let slot_index = absolute_index / 16;
            let relative_index = absolute_index % 16;

            // pop group from stack and add to the slot
            bitmap_list.inner[relative_index] = read_bitmaps.pop().unwrap();

            // If the last element of the slot was entered or the list is exhausted, write and flush the slot
            if relative_index == 15 || read_bitmaps.is_empty() {
                let key = BitmapListSlotKey {
                    market_index: self.market_index,
                    index: slot_index as u16,
                };

                bitmap_list.write_to_slot(slot_storage, &key);

                // reset to empty slot
                bitmap_list = BitmapListSlot::default();
            }
        }

        self.size += 1;
    }
}

#[derive(Default)]
pub struct BitmapListSlot {
    pub inner: [u16; 16],
}

impl BitmapListSlot {
    /// Load from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &BitmapListSlotKey) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        BitmapListSlot::decode(slot)
    }

    /// Decode from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        BitmapListSlot {
            inner: unsafe { core::mem::transmute::<[u8; 32], [u16; 16]>(slot) },
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        unsafe { core::mem::transmute::<[u16; 16], [u8; 32]>(self.inner) }
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &BitmapListSlotKey) {
        let bytes = self.encode();
        slot_storage.sstore(&key.get_key(), &bytes);
    }
}

/// Slot index to fetch a BitmapListSlot
///
/// The max number of BitmapListSlots can be 2^16 / 16 - 1 = 2^12 - 1
///
pub struct BitmapListSlotKey {
    /// The market index
    pub market_index: u8,

    /// Index of the BitmapListSlot, max 2^12 - 1
    pub index: u16,
}

impl SlotKey for BitmapListSlotKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = TICK_GROUP_LIST_KEY_SEED;
        key[1] = self.market_index;
        key[2..4].copy_from_slice(&self.index.to_le_bytes());

        key
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

        let bitmap_list_slot = BitmapListSlot::decode(bytes);
        assert_eq!(
            bitmap_list_slot.inner,
            [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_decode() {
        let bitmap_list_slot = BitmapListSlot {
            inner: [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        assert_eq!(
            bitmap_list_slot.encode(),
            [
                0, 0, 1, 0, 2, 0, 3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_insert_for_bids() {
        let mut slot_storage = SlotStorage::new();

        let market_index = 0;
        let side = Side::Bid;

        let mut list = BitmapList {
            market_index,
            side,
            size: 0,
        };

        // 1. insert first item
        list.insert(&mut slot_storage, 1);

        assert_eq!(list.size, 1);
        let key = BitmapListSlotKey {
            market_index,
            index: 0,
        };

        let mut bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert second item
        list.insert(&mut slot_storage, 2);

        assert_eq!(list.size, 2);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert third item
        list.insert(&mut slot_storage, 4);

        assert_eq!(list.size, 3);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [1, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert forth item in the middle
        list.insert(&mut slot_storage, 3);

        assert_eq!(list.size, 4);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_insert_for_asks() {
        let mut slot_storage = SlotStorage::new();

        let market_index = 0;
        let side = Side::Ask;

        let mut list = BitmapList {
            market_index,
            side,
            size: 0,
        };

        // 1. insert first item
        list.insert(&mut slot_storage, 4);

        assert_eq!(list.size, 1);
        let key = BitmapListSlotKey {
            market_index,
            index: 0,
        };

        let mut bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert second item
        list.insert(&mut slot_storage, 2);

        assert_eq!(list.size, 2);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert third item
        list.insert(&mut slot_storage, 1);

        assert_eq!(list.size, 3);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 3. insert forth item in the middle
        list.insert(&mut slot_storage, 3);

        assert_eq!(list.size, 4);

        bitmap_list_slot = BitmapListSlot::new_from_slot(&slot_storage, &key);
        assert_eq!(
            bitmap_list_slot.inner,
            [4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_insert_across_multiple_slots_for_bids() {
        let mut slot_storage = SlotStorage::new();

        let market_index = 0;
        let side = Side::Bid;

        let mut list = BitmapList {
            market_index,
            side,
            size: 16,
        };

        let key_0 = BitmapListSlotKey {
            market_index,
            index: 0,
        };
        let key_1 = BitmapListSlotKey {
            market_index,
            index: 1,
        };

        let initial_active_bitmaps = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17];

        let slot_0_initial = BitmapListSlot {
            inner: initial_active_bitmaps,
        }
        .encode();

        slot_storage.sstore(&key_0.get_key(), &slot_0_initial);

        // 1. insert element at end
        list.insert(&mut slot_storage, 18);
        assert_eq!(list.size, 17);

        let mut bitmap_list_slot_0 = BitmapListSlot::new_from_slot(&slot_storage, &key_0);
        let mut bitmap_list_slot_1 = BitmapListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            bitmap_list_slot_0.inner,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17,]
        );

        assert_eq!(
            bitmap_list_slot_1.inner,
            [18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert element in middle
        list.insert(&mut slot_storage, 16);
        assert_eq!(list.size, 18);

        bitmap_list_slot_0 = BitmapListSlot::new_from_slot(&slot_storage, &key_0);
        bitmap_list_slot_1 = BitmapListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            bitmap_list_slot_0.inner,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,]
        );

        assert_eq!(
            bitmap_list_slot_1.inner,
            [17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_insert_across_multiple_slots_for_asks() {
        let mut slot_storage = SlotStorage::new();

        let market_index = 0;
        let side = Side::Ask;

        let mut list = BitmapList {
            market_index,
            side,
            size: 16,
        };

        let key_0 = BitmapListSlotKey {
            market_index,
            index: 0,
        };
        let key_1 = BitmapListSlotKey {
            market_index,
            index: 1,
        };

        let initial_active_bitmaps = [18, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

        let slot_0_initial = BitmapListSlot {
            inner: initial_active_bitmaps,
        }
        .encode();

        slot_storage.sstore(&key_0.get_key(), &slot_0_initial);

        // 1. insert element at end
        list.insert(&mut slot_storage, 1);
        assert_eq!(list.size, 17);

        let mut bitmap_list_slot_0 = BitmapListSlot::new_from_slot(&slot_storage, &key_0);
        let mut bitmap_list_slot_1 = BitmapListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            bitmap_list_slot_0.inner,
            [18, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2,]
        );

        assert_eq!(
            bitmap_list_slot_1.inner,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );

        // 2. insert element in middle
        list.insert(&mut slot_storage, 17);
        assert_eq!(list.size, 18);

        bitmap_list_slot_0 = BitmapListSlot::new_from_slot(&slot_storage, &key_0);
        bitmap_list_slot_1 = BitmapListSlot::new_from_slot(&slot_storage, &key_1);
        assert_eq!(
            bitmap_list_slot_0.inner,
            [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3,]
        );

        assert_eq!(
            bitmap_list_slot_1.inner,
            [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }
}
