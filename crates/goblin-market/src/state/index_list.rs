/// Index List is a contiguous list of active outer indices of bitmap groups. A bitmap group
/// is active if at least one of its ticks has a resting order, live or expired.
///
/// There is one list for bids and another for asks. Elements are sorted in descending
/// order for asks and in ascending order for bids, i.e. elements at centre of the list
/// are at the end of the list. This allows cheaper updates because these elements are read most.
///
/// Conversely updates at beginning of the list, i.e. furthest from middle of the orderbook
/// cost more because the entire list must be shifted right.
///
/// Each tick group index is made of 2 bits in big endian format. This means that
/// each ListItem contains 16 outer indices.
use crate::state::{ArbContext, ContextActions, OuterIndex, Side, SlotKey, LIST_KEY_SEED};

/// Slot key to fetch a ListSlot
///
/// The number of allowed indices per list is u16::MAX. Since one slot contains 16 indices,
/// the max number of slots are 2^16 / 16 - 1 = 2^12 - 1 = 4095
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
    pub fn new_from_slot(slot_storage: &ArbContext, key: ListKey) -> Self {
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

    pub fn write_to_slot(&self, slot_storage: &mut ArbContext, key: &ListKey) {
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
}
