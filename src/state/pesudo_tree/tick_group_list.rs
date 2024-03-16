use crate::state::{SlotActions, SlotKey, SlotStorage};

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
/// each TickGroupList element contains 16 tick group indices.
///
/// TODO handle insertion deletion inside pseudo_tree
pub struct TickGroupList {
    pub inner: [u16; 16]
}

impl TickGroupList {
    /// Load TickGroupList from slot storage
    pub fn new_from_slot(slot_storage: &SlotStorage, key: &TickGroupListKey) -> Self {
        let slot = slot_storage.sload(&key.get_key());

        TickGroupList::decode(slot)
    }

    /// Decode TickGroupList from slot
    pub fn decode(slot: [u8; 32]) -> Self {
        TickGroupList {
            inner: unsafe { core::mem::transmute::<[u8; 32], [u16; 16]>(slot) }
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        unsafe { core::mem::transmute::<[u16; 16], [u8; 32]>(self.inner) }
    }
}

/// Slot index to fetch a TickGroupList
///
/// The max number of TickGroupList items can be 2^16 / 16 - 1 = 2^12 - 1
///
pub struct TickGroupListKey {
    /// The market index
    market_index: u8,

    /// Index of the TickGroupList, max 2^12 - 1
    index: u16,
}

impl SlotKey for TickGroupListKey {
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
            0, 0, 1, 0, 2, 0, 3, 0,
            0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let tick_group_list = TickGroupList::decode(bytes);
        assert_eq!(tick_group_list.inner, [
            0, 1, 2, 3,
            256, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ]);
    }

    #[test]
    fn test_decode() {
        let tick_group_list = TickGroupList {
            inner: [
                0, 1, 2, 3,
                256, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
            ]
        };

        assert_eq!(tick_group_list.encode(), [
            0, 0, 1, 0, 2, 0, 3, 0,
            0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]);
    }
}
