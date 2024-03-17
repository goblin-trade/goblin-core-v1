use crate::state::{slot_storage, Side, SlotActions, SlotKey, SlotStorage};
use alloc::vec::Vec;

const TICK_GROUP_LIST_KEY_SEED: u8 = 0;

/// A contiguous list of active tick group indexes. A tick group is active
/// if at least one of its ticks has a resting order, live or expired.
///
/// The elements are sorted in descending order for asks and in ascending order for bids.
/// Since ticks in middle of the orderbook are accessed most, having them at the end
/// allows cheaper updates. Updates at beginning of the list, i.e. furthest from middle
/// of the orderbook cost more because the entire list must be shifted right.
///
/// Each tick group index is made of 2 bits in little endian format. This means that
/// each ActiveTickGroup element contains 16 tick group indices.
///
/// TODO handle insertion deletion inside pseudo_tree
pub struct TickGroupList {
    /// The market index
    pub market_index: u8,

    /// Bid or ask
    pub side: Side,

    /// Number of tick groups saved in slots
    pub size: u16,
}

impl TickGroupList {
    /// Sort in ascending order for bids and in descending order for asks,
    /// such that elements at middle of the orderbook are at the end of the list.
    pub fn ascending(&self) -> bool {
        self.side == Side::Bid
    }

    /// Add a tick_group to the tick group list
    /// We iterate from the end of the list to find the right index to insert at
    ///
    /// Since a tick_group_item is made of 16 items per slot, we need to rewrite
    /// the slot into which the inserted item falls, and also rewrite elements on the
    /// right to account for a right shift.
    ///
    /// We must externally ensure that the inserted item is not already present.
    ///
    pub fn insert(&mut self, slot_storage: &mut SlotStorage, new_group: u16) {
        // Save read tick groups that fall to the right of the inserted item in memory.
        // These elements will be right shifted and written to slot.
        let mut read_groups = Vec::<u16>::new();

        let mut tick_group_slot = TickGroupItem::default();

        // TODO handle empty case
        for i in (0..self.size).rev() {
            let slot_index = i / 16;
            let group_index = i % 16;

            // Fetch slot if this is the first time, or we have exhausted the slot
            if i == self.size - 1 || group_index == 0 {
                let key = TickGroupItemKey {
                    market_index: self.market_index,
                    index: slot_index,
                };

                tick_group_slot = TickGroupItem::new_from_slot(slot_storage, &key);
            }

            // the current group
            let group = tick_group_slot.inner[group_index as usize];

            // check whether new_group is to be inserted after group
            if (self.ascending() && group < new_group) || (!self.ascending() && group > new_group) {
                read_groups.push(group);
            } else {
                let mut group_slot_to_write = TickGroupItem::default();

                // save elements on left of new_group
                let groups_on_left = &tick_group_slot.inner[0..(group_index as usize)];
                group_slot_to_write.inner[0..(group_index as usize)]
                    .copy_from_slice(groups_on_left);

                // right shift and saving logic

                // push the element to insert at top of the stack
                read_groups.push(new_group);

                for j in 0..read_groups.len() {
                    let absolute_index = i as usize + j;
                    let slot_index = absolute_index / 16;
                    let group_index = absolute_index % 16;

                    // pop group from stack and add to the slot
                    group_slot_to_write.inner[group_index] = read_groups.pop().unwrap();

                    // If the last element of the slot was entered, write and flush the slot
                    if group_index == 15 {
                        let key = TickGroupItemKey {
                            market_index: self.market_index,
                            index: slot_index as u16,
                        };

                        group_slot_to_write.write_to_slot(slot_storage, &key);

                        // reset to empty slot
                        group_slot_to_write = TickGroupItem::default();
                    }
                }

                self.size += 1;
                return;
            }
        }
    }
}

#[derive(Default)]
pub struct TickGroupItem {
    pub inner: [u16; 16],
}

impl TickGroupItem {
    /// Load ActiveTickGroup from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &TickGroupItemKey) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        TickGroupItem::decode(slot)
    }

    /// Decode ActiveTickGroup from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        TickGroupItem {
            inner: unsafe { core::mem::transmute::<[u8; 32], [u16; 16]>(slot) },
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        unsafe { core::mem::transmute::<[u16; 16], [u8; 32]>(self.inner) }
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, key: &TickGroupItemKey) {
        let bytes = self.encode();
        slot_storage.sstore(&key.get_key(), &bytes);
    }
}

/// Slot index to fetch a TickGroupItem
///
/// The max number of TickGroupItems can be 2^16 / 16 - 1 = 2^12 - 1
///
pub struct TickGroupItemKey {
    /// The market index
    market_index: u8,

    /// Index of the TickGroupSlot, max 2^12 - 1
    index: u16,
}

impl SlotKey for TickGroupItemKey {
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

        let tick_group_list = TickGroupItem::decode(bytes);
        assert_eq!(
            tick_group_list.inner,
            [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_decode() {
        let tick_group_list = TickGroupItem {
            inner: [0, 1, 2, 3, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        assert_eq!(
            tick_group_list.encode(),
            [
                0, 0, 1, 0, 2, 0, 3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_insert() {
        let mut slot_storage = SlotStorage::new();

        let mut list = TickGroupList {
            market_index: 0,
            side: Side::Bid,
            size: 0,
        };

        list.insert(&mut slot_storage, 1);

        assert_eq!(list.size, 1);
    }
}
