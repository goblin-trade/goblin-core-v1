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
use crate::{
    program::{GoblinError, GoblinResult, IndexNotInList, IndicesNotInOrder},
    require,
    state::{OuterIndex, Side, SlotActions, SlotKey, SlotStorage, LIST_KEY_SEED},
};
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
#[derive(Clone, Copy)]
pub struct ListKey {
    /// Index of the ListSlot, max 2^12 - 1
    pub index: u16,

    /// Whether bid or index slot
    pub side: Side,
}

impl SlotKey for ListKey {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = LIST_KEY_SEED;
        key[1] = (self.side == Side::Bid) as u8;
        key[2..4].copy_from_slice(&self.index.to_be_bytes());

        key
    }
}

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

    // Sets a placeholder value for a ListSlot that has been completely traversed.
    // We save gas by not writing 0 to slot, that way the slot is not cleared.
    //
    // TODO check behavior when order_iterator removes items. Empty slots are being cleared.
    pub fn clear(&mut self) {
        self.inner = [u16::MAX; 16];
    }

    pub fn clear_index(&mut self, list_key: &ListKey) {
        self.inner[list_key.index as usize] = u16::MAX;
    }
}

/// High level structure for the index list with getter and setter functions
pub struct IndexList {
    /// Whether list is for bids or asks
    pub side: Side,

    /// Number of active outer indices in the list that have not been cached
    /// The number of slots is found as size / 16 and index of an item is found as size % 16
    pub size: u16,

    /// Outer indices cached to memory. Used to temporarily store values when
    /// removing an index. Values on right side of the removed value are stored here.
    pub cached_values: Vec<OuterIndex>,

    /// Cached outer index closest to the centre
    pub cached_best_outer_index: Option<OuterIndex>,

    /// Cached current slot
    pub cached_slot: Option<ListSlot>,

    /// The current index when iterating through values
    pub current_index: Option<u16>,
}

impl IndexList {
    pub fn new(side: Side, size: u16) -> IndexList {
        IndexList {
            side,
            size,
            cached_values: Vec::new(),

            cached_best_outer_index: None,
            cached_slot: None,
            current_index: None,
        }
    }

    /// Get the best outer index. This is the outermost value in this list with the greatest index.
    ///
    /// If remove() was called, only call this function after performing write_to_slot()
    ///
    pub fn get_best_outer_index(&self, slot_storage: &SlotStorage) -> OuterIndex {
        if self.cached_best_outer_index.is_some() {
            return self.cached_best_outer_index.unwrap();
        } else {
            // There is no cache if no index was removed from the list. We need to read from slot.
            let slot_index = (self.size - 1) / 16;
            let relative_index = (self.size - 1) as usize % 16;

            #[cfg(test)]
            println!(
                "slot_index {}, relative_index {}",
                slot_index, relative_index
            );

            let key = ListKey {
                index: slot_index,
                side: self.side,
            };
            let current_slot = ListSlot::new_from_slot(slot_storage, key);

            #[cfg(test)]
            println!("got slot  {:?}", current_slot.inner);

            current_slot.get(relative_index)
        }
    }

    /// Remove an outer index from the list
    ///
    /// The updated list is not stored. Call write_to_slot() to persist.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `value_to_remove` - The outer index to remove
    ///
    pub fn remove(
        &mut self,
        slot_storage: &SlotStorage,
        value_to_remove: OuterIndex,
    ) -> GoblinResult<()> {
        if let Some(outermost_index) = self.cached_values.last() {
            require!(
                (self.side == Side::Bid && value_to_remove < *outermost_index)
                    || (self.side == Side::Ask && value_to_remove > *outermost_index),
                GoblinError::IndicesNotInOrder(IndicesNotInOrder {})
            );
        }

        // Keep reading from index list till the value_to_remove is found
        // Cache items on the right
        while self.size > 0 {
            self.size -= 1;

            let slot_index = self.size / 16;
            let relative_index = self.size as usize % 16;

            // Load a new slot and cache it
            if self.cached_slot.is_none() || relative_index == 15 {
                let key = ListKey {
                    index: slot_index,
                    side: self.side,
                };
                self.cached_slot = Some(ListSlot::new_from_slot(slot_storage, key));
            }

            let current_slot = self.cached_slot.as_mut().unwrap();
            let current_value = current_slot.get(relative_index);

            // Remove elements from current slot. Move them to `cached_values` stash
            current_slot.set(relative_index, OuterIndex::new(0));

            if current_value == value_to_remove {
                break;
            } else if self.size != 0 {
                self.cached_values.push(current_value);
            } else {
                return Err(GoblinError::IndexNotInList(IndexNotInList {}));
            }
        }

        Ok(())
    }

    /// Write the cached index list to slot. The list is reconstructed from the current
    /// cached slot and the cached stash of values.
    ///
    /// No-op if cached_slot is None, i.e. if remove() was never called.
    ///
    pub fn write_to_slot(&mut self, slot_storage: &mut SlotStorage) {
        // Absolute index of the value to write

        if let Some(cached_slot) = self.cached_slot.as_mut() {
            // No stash case. Simply write the cached slot
            if self.cached_values.is_empty() {
                let slot_key = ListKey {
                    index: self.size / 16,
                    side: self.side,
                };

                // Write cached slot
                cached_slot.write_to_slot(slot_storage, &slot_key);
            } else {
                // Stash present. Write values one by one.
                for index in (self.size as usize)..(self.size as usize + self.cached_values.len()) {
                    let slot_index = index / 16;
                    let relative_index = index % 16;

                    let value_to_add = self.cached_values.pop().unwrap();
                    cached_slot.set(relative_index, value_to_add);

                    #[cfg(test)]
                    println!("writing {} at index {}", value_to_add.as_u16(), index);

                    if self.cached_values.is_empty() || relative_index == 15 {
                        // Write
                        let slot_key = ListKey {
                            index: slot_index as u16,
                            side: self.side,
                        };
                        cached_slot.write_to_slot(slot_storage, &slot_key);

                        // Clear the in-memory slot for further writes
                        cached_slot.inner = [0u16; 16];
                    }

                    self.size += 1;

                    // Cache the best index after write is complete. This will be used in get_best_outer_index()
                    // cached_best_outer_index is not stored if there was no cached stash
                    if self.cached_values.is_empty() {
                        self.cached_best_outer_index = Some(value_to_add);
                    }
                }
            }
        }

        self.cached_slot = None;
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
    fn test_remove_outermost() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let mut list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        let outer_index = index_list.get_best_outer_index(&slot_storage);
        assert_eq!(outer_index.as_u16(), 3);

        // Remove outermost value. Nothing gets pushed to cache
        index_list
            .remove(&slot_storage, OuterIndex::new(3))
            .unwrap();
        assert_eq!(index_list.size, 3);
        assert_eq!(index_list.cached_values.len(), 0);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        index_list.write_to_slot(&mut slot_storage);
        assert_eq!(index_list.size, 3);
        assert_eq!(index_list.cached_values.len(), 0);
        // No cached outer index since the stash was never populated
        assert!(index_list.cached_best_outer_index.is_none());
        assert!(index_list.cached_slot.is_none());

        list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(
            list_slot.inner,
            [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_remove_inner() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let mut list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        // Remove an inner element. Values to the right of this value are cached
        index_list
            .remove(&slot_storage, OuterIndex::new(1))
            .unwrap();

        assert_eq!(index_list.size, 1);

        let cached_values: Vec<u16> = index_list
            .cached_values
            .iter()
            .map(|cached| cached.as_u16())
            .collect();
        assert_eq!(cached_values, vec![3, 2]);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        index_list.write_to_slot(&mut slot_storage);
        assert_eq!(index_list.size, 3);
        assert_eq!(index_list.cached_values.len(), 0);
        assert_eq!(index_list.cached_best_outer_index.unwrap().as_u16(), 3);
        assert!(index_list.cached_slot.is_none());

        list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(
            list_slot.inner,
            [0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_remove_inner_zero() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let mut list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        // Remove an inner element. Values to the right of this value are cached
        index_list
            .remove(&slot_storage, OuterIndex::new(0))
            .unwrap();
        assert_eq!(index_list.size, 0);

        let cached_values: Vec<u16> = index_list
            .cached_values
            .iter()
            .map(|cached| cached.as_u16())
            .collect();
        assert_eq!(cached_values, vec![3, 2, 1]);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        index_list.write_to_slot(&mut slot_storage);
        assert_eq!(index_list.size, 3);
        assert_eq!(index_list.cached_values.len(), 0);
        assert_eq!(index_list.cached_best_outer_index.unwrap().as_u16(), 3);
        assert!(index_list.cached_slot.is_none());

        list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(
            list_slot.inner,
            [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_remove_multiple_then_write() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let mut list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        index_list
            .remove(&slot_storage, OuterIndex::new(2))
            .unwrap();

        index_list
            .remove(&slot_storage, OuterIndex::new(1))
            .unwrap();

        assert_eq!(index_list.size, 1);

        let cached_values: Vec<u16> = index_list
            .cached_values
            .iter()
            .map(|cached| cached.as_u16())
            .collect();
        assert_eq!(cached_values, vec![3]);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        index_list.write_to_slot(&mut slot_storage);

        assert_eq!(index_list.size, 2);
        assert_eq!(index_list.cached_values.len(), 0);
        assert_eq!(index_list.cached_best_outer_index.unwrap().as_u16(), 3);
        assert!(index_list.cached_slot.is_none());

        list_slot = ListSlot::new_from_slot(&slot_storage, list_key);
        assert_eq!(
            list_slot.inner,
            [0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_remove_single_across_two_slots() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        let mut list_slot_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_key_0 = ListKey { index: 0, side };

        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);

        let mut list_slot_1 = ListSlot {
            inner: [16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key_1 = ListKey { index: 1, side };

        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        let size = 19;
        let mut index_list = IndexList::new(side, size);

        index_list
            .remove(&slot_storage, OuterIndex::new(15))
            .unwrap();

        assert_eq!(index_list.size, 15);

        let cached_values: Vec<u16> = index_list
            .cached_values
            .iter()
            .map(|cached| cached.as_u16())
            .collect();
        assert_eq!(cached_values, vec![18, 17, 16]);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0]
        );

        index_list.write_to_slot(&mut slot_storage);

        assert_eq!(index_list.size, 18);
        assert_eq!(index_list.cached_values.len(), 0);
        assert_eq!(index_list.cached_best_outer_index.unwrap().as_u16(), 18);
        assert!(index_list.cached_slot.is_none());

        list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        assert_eq!(
            list_slot_0.inner,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16]
        );

        list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        assert_eq!(
            list_slot_1.inner,
            [17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_remove_multiple_across_two_slots() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        let mut list_slot_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_key_0 = ListKey { index: 0, side };

        list_slot_0.write_to_slot(&mut slot_storage, &list_key_0);

        let mut list_slot_1 = ListSlot {
            inner: [16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key_1 = ListKey { index: 1, side };

        list_slot_1.write_to_slot(&mut slot_storage, &list_key_1);

        let size = 19;
        let mut index_list = IndexList::new(side, size);

        index_list
            .remove(&slot_storage, OuterIndex::new(17))
            .unwrap();

        index_list
            .remove(&slot_storage, OuterIndex::new(14))
            .unwrap();

        assert_eq!(index_list.size, 14);

        let cached_values: Vec<u16> = index_list
            .cached_values
            .iter()
            .map(|cached| cached.as_u16())
            .collect();
        assert_eq!(cached_values, vec![18, 16, 15]);
        assert!(index_list.cached_best_outer_index.is_none());

        assert_eq!(
            index_list.cached_slot.as_ref().unwrap().inner,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 0]
        );

        index_list.write_to_slot(&mut slot_storage);

        assert_eq!(index_list.size, 17);
        assert_eq!(index_list.cached_values.len(), 0);
        assert_eq!(index_list.cached_best_outer_index.unwrap().as_u16(), 18);
        assert!(index_list.cached_slot.is_none());

        list_slot_0 = ListSlot::new_from_slot(&slot_storage, list_key_0);
        assert_eq!(
            list_slot_0.inner,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16]
        );

        list_slot_1 = ListSlot::new_from_slot(&slot_storage, list_key_1);
        assert_eq!(
            list_slot_1.inner,
            [18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_remove_multiple_fails_if_wrong_order_for_bids() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        index_list
            .remove(&slot_storage, OuterIndex::new(2))
            .unwrap();

        assert!(index_list
            .remove(&slot_storage, OuterIndex::new(3))
            .is_err());
    }

    #[test]
    fn test_remove_multiple_fails_if_wrong_order_for_asks() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Ask;

        // Insert initial values in list
        let list_slot = ListSlot {
            inner: [10, 9, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        index_list
            .remove(&slot_storage, OuterIndex::new(9))
            .unwrap();

        assert!(index_list
            .remove(&slot_storage, OuterIndex::new(8))
            .is_err());
    }

    #[test]
    fn test_remove_fails_if_value_not_found() {
        let mut slot_storage = SlotStorage::new();
        let side = Side::Bid;

        // Insert initial values in list
        let list_slot = ListSlot {
            inner: [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let list_key = ListKey { index: 0, side };

        list_slot.write_to_slot(&mut slot_storage, &list_key);

        let size = 4;
        let mut index_list = IndexList::new(side, size);

        assert!(index_list
            .remove(&slot_storage, OuterIndex::new(100))
            .is_err());
    }
}
