use crate::{
    quantities::Ticks,
    state::{RestingOrder, Side, SlotStorage},
};

use super::{
    inner_indices, BitmapGroup, InnerIndex, ListKey, ListSlot, OrderId, RestingOrderIndex,
    SlotRestingOrder,
};

pub fn process_resting_orders(
    slot_storage: &mut SlotStorage,
    side: Side,
    outer_index_count: u16,
    best_price: Ticks,
    lambda_function: fn(resting_order: &mut SlotRestingOrder) -> bool,
) {
    // let mut current_outer_index_count = outer_index_count;
    let mut slot_index = (outer_index_count - 1) / 16;
    let mut relative_index = (outer_index_count - 1) % 16;

    let mut previous_inner_index = Some(best_price.inner_index());

    // 1. Loop through index slots
    loop {
        let list_key = ListKey { index: slot_index };
        let mut slot = ListSlot::new_from_slot(slot_storage, list_key);

        // 2. Loop through bitmap groups using relative index
        loop {
            let outer_index = slot.get(relative_index as usize);
            let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            // 3. Loop through bitmaps
            for i in inner_indices(side, previous_inner_index) {
                let inner_index = InnerIndex::new(i);
                let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);

                let price_in_ticks = Ticks::from_indices(outer_index, inner_index);

                // 4. Loop through resting orders in the bitmap
                for j in 0..8 {
                    let resting_order_index = RestingOrderIndex::new(j);
                    let order_present = bitmap.order_present(resting_order_index);

                    if order_present {
                        let order_id = OrderId {
                            price_in_ticks,
                            resting_order_index,
                        };
                        let mut resting_order =
                            SlotRestingOrder::new_from_slot(slot_storage, order_id);

                        let continue_reads = lambda_function(&mut resting_order);

                        if !continue_reads {
                            // If reads are to be stopped, emit break and leave the loops.
                            // Bitmap groups and index list must be updated since the update statements
                            // at end of respective loops will not be encountered.
                            //
                            if resting_order.size() == 0 {
                                // Special case- The resting order as well as the input amount is
                                // completely exhausted. We need to find the next active resting order
                                // and write its price in market state.
                            }
                        }

                        bitmap.clear(&resting_order_index);
                    }
                }
            }
            // Previous inner index is only used for the first active tick
            if previous_inner_index.is_some() {
                previous_inner_index = None;
            }

            bitmap_group.write_to_slot(slot_storage, &outer_index);

            if relative_index == 0 {
                break;
            }
            relative_index -= 1;
        }

        // All orders for the slot index have been purged
        // Clear this slot.

        slot.clear();
        slot.write_to_slot(slot_storage, &list_key);

        if slot_index == 0 {
            break;
        }
        // Move to the next ListSlot. Reset the relative index.
        slot_index -= 1;
        relative_index = 15;
    }
}
