use crate::state::{InnerIndex, OrderId, OuterIndex, Side, SlotStorage, TickIndices};

use super::{BitmapRemover, IndexListReader};

pub struct BitmapReader {
    pub bitmap_remover: BitmapRemover,

    /// Iterator to read saved values from list
    pub index_list_reader: IndexListReader,
}

impl BitmapReader {
    pub fn new(outer_index_count: u16, side: Side) -> Self {
        BitmapReader {
            bitmap_remover: BitmapRemover::new(),
            index_list_reader: IndexListReader::new(outer_index_count, side),
        }
    }

    /// Checks whether an order is present at the given order ID
    /// Externally ensure that order IDs move away from the centre
    pub fn order_present(&mut self, slot_storage: &mut SlotStorage, order_id: OrderId) -> bool {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        // Set the outer index in bitmap remover
        if self.bitmap_remover.last_outer_index != Some(outer_index) {
            // Traverse the index list to search for outer index
            let outer_index_found = self
                .index_list_reader
                .find_outer_index(slot_storage, outer_index);

            if !outer_index_found {
                return false;
            }

            self.bitmap_remover
                .set_outer_index(slot_storage, outer_index);
        }

        // Now check in bitmap group
        return self
            .bitmap_remover
            .order_present(inner_index, resting_order_index);
    }
}
