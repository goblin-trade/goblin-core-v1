use crate::{
    quantities::Ticks,
    state::{
        slot_storage, BitmapGroup, InnerIndex, ListKey, ListSlot, OrderId, OuterIndex,
        RestingOrderIndex, Side, SlotRestingOrder, SlotStorage,
    },
};

#[derive(Clone, Copy)]
pub struct IteratedRestingOrder {
    pub resting_order: SlotRestingOrder,
    pub order_id: OrderId, // pub outer_index: OuterIndex,
                           // pub inner_index: InnerIndex
}

/// Iterate over resting orders in a book
///
/// Whenever next is called, the preceding value should be deleted
// #[derive(Clone)]
pub struct OrderIterator<'a> {
    pub slot_storage: &'a mut SlotStorage,

    pub side: Side,

    // pub starting_price: Ticks,
    pub last_item: Option<IteratedRestingOrder>,

    pub cached_list_slot: Option<ListSlot>,

    pub bitmap_group: Option<BitmapGroup>,

    // These variables update as we traverse
    pub outer_index_count: u16,

    pub inner_index: Option<InnerIndex>,

    pub bit_index: Option<RestingOrderIndex>,
}

// impl<'a> OrderIterator<'a> {
//     pub fn new(slot_storage: &'a mut SlotStorage) -> Self {
//         OrderIterator { slot_storage }
//     }
// }

impl<'a> OrderIterator<'a> {
    pub fn new(
        slot_storage: &'a mut SlotStorage,
        side: Side,
        outer_index_count: u16,
        best_price: Ticks,
    ) -> Self {
        let inner_index = best_price.inner_index();

        OrderIterator {
            slot_storage,
            side,
            last_item: None,
            cached_list_slot: None,
            bitmap_group: None,
            outer_index_count,
            inner_index: Some(inner_index),
            bit_index: None,
        }
    }
}

impl Iterator for OrderIterator<'_> {
    type Item = IteratedRestingOrder;

    fn next(&mut self) -> Option<Self::Item> {
        if self.outer_index_count == 0 {
            return None;
        }

        // Better to update state externally in loop body?
        if let Some(IteratedRestingOrder {
            resting_order,
            order_id,
        }) = &mut self.last_item
        {
            // Remove this item
            resting_order.clear_order();
            resting_order.write_to_slot(self.slot_storage, order_id);

            // Update bitmap
            let mut bitmap_group = self.bitmap_group.unwrap();
            let mut bitmap = bitmap_group.get_bitmap_mut(&order_id.price_in_ticks.inner_index());
            bitmap.flip(&order_id.resting_order_index);

            // Not optimal- should only write when group changes
            bitmap_group.write_to_slot(self.slot_storage, &order_id.price_in_ticks.outer_index());
        }

        let outer_index_position = self.outer_index_count - 1;

        // Index of a slot item. A slot item holds upto 16 outer indices
        let slot_index = outer_index_position / 16;

        // Relative index inside a slot item
        let relative_index = outer_index_position as usize % 16;

        // Read the outer index

        // Load list slot if this is the first iteration or if values in the cached slot are
        // completely read
        if self.cached_list_slot.is_none() || relative_index == 15 {
            let list_slot_key = ListKey { index: slot_index };
            self.cached_list_slot = Some(ListSlot::new_from_slot(self.slot_storage, list_slot_key));
        }

        let list_slot = self.cached_list_slot.unwrap();

        // Read outer index from list
        let outer_index = list_slot.get(relative_index);

        // Only update if new outer index was read
        if self.bitmap_group.is_none() {
            self.bitmap_group = Some(BitmapGroup::new_from_slot(self.slot_storage, outer_index));
        }

        // Read group with inner index
        let bitmap_group = self.bitmap_group.unwrap();

        if self.inner_index.is_none() {
            // Guaranteed to hold a value
            self.inner_index = bitmap_group.best_active_index(self.side, None);
        }
        let inner_index = self.inner_index.unwrap();

        let bitmap = bitmap_group.get_bitmap(&inner_index);

        let bit_index = self.bit_index.unwrap_or(bitmap.best_free_index(0).unwrap());

        let order_id = OrderId {
            price_in_ticks: Ticks::from_indices(outer_index, inner_index),
            resting_order_index: bit_index,
        };

        let resting_order = SlotRestingOrder::new_from_slot(self.slot_storage, order_id);

        let item = IteratedRestingOrder {
            resting_order,
            order_id,
        };

        self.last_item = Some(item);

        // Update variables to read the next item
        // The next state variables cannot be known without the current ones, threfore
        // these are updated last

        self.bit_index = bitmap.best_free_index(bit_index.as_u8() + 1);

        // If bit index is exhausted, set to None and advance inner index
        if self.bit_index.is_none() {
            self.inner_index = bitmap_group.best_active_index(self.side, self.inner_index);
        }

        // If bitmap group is exhausted, advance outer index (decrement count)
        if self.inner_index.is_none() {
            // Bitmap group is exhausted, write to slot
            bitmap_group.write_to_slot(self.slot_storage, &outer_index);
            self.bitmap_group = None;

            self.outer_index_count -= 1;
        }

        Some(item)
    }
}
