use crate::{
    quantities::Ticks,
    state::{market_state, MarketState, RestingOrder, Side, SlotStorage},
};

use super::{
    index_list, inner_indices, BitmapGroup, InnerIndex, ListKey, ListSlot, OrderId,
    RestingOrderIndex, SlotRestingOrder,
};

pub fn process_resting_orders(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    side: Side,
    // outer_index_count: u16,
    // best_price: Ticks,
    lambda_function: fn(resting_order: &mut SlotRestingOrder) -> bool,
) {
    let outer_index_count = market_state.outer_index_length(side);
    let best_price = market_state.best_price(side);

    // let mut current_outer_index_count = outer_index_count;
    let mut slot_index = (outer_index_count - 1) / 16;
    let mut relative_index = (outer_index_count - 1) % 16;

    let mut previous_inner_index = Some(best_price.inner_index());

    let mut continue_reads = true;

    // 1. Loop through index slots
    'outer: loop {
        let list_key = ListKey { index: slot_index };
        let mut list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut list_slot_traversed = false;

        // 2. Loop through bitmap groups using relative index
        loop {
            let outer_index = list_slot.get(relative_index as usize);
            let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            let mut bitmap_group_traversed = false;

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
                        if !continue_reads {
                            bitmap_group.write_to_slot(slot_storage, &outer_index);
                            list_slot.write_to_slot(slot_storage, &list_key);
                            market_state.set_best_price(side, price_in_ticks);

                            return;
                        }

                        let order_id = OrderId {
                            price_in_ticks,
                            resting_order_index,
                        };
                        let mut resting_order =
                            SlotRestingOrder::new_from_slot(slot_storage, order_id);

                        continue_reads = lambda_function(&mut resting_order);

                        // The input amount is consumed, exit.
                        // Traversed Bitmap groups and ListSlots have been written already
                        if resting_order.size() != 0 {
                            market_state.set_best_price(side, price_in_ticks);

                            return;
                        }
                        // if !continue_reads {
                        //     // If reads are to be stopped, emit break and leave the loops.
                        //     // Bitmap groups and index list must be updated since the update statements
                        //     // at end of respective loops will not be encountered.

                        //     // States to update
                        //     // - bitmap group
                        //     // - ListSlot
                        //     // - MarketState: If resting_order.size() == 0
                        //     //
                        //     // resting_order.size() == 0 case is more complex. The bitmap group needs
                        //     // to be updated. If the bitmap group closed then ListSlot will be updated too.
                        //     // Further we need to discover the next active tick and write it to market state.
                        //     // This discovery may require multiple iterations.

                        //     if bitmap_group_traversed {
                        //         if resting_order.size() == 0 {
                        //             bitmap.clear(&resting_order_index);
                        //         }
                        //         bitmap_group.write_to_slot(slot_storage, &outer_index);
                        //     }

                        //     if resting_order.size() == 0 {
                        //         // Special case- The resting order as well as the input amount is
                        //         // completely exhausted. We need to find the next active resting order
                        //         // and write its price in market state.
                        //     }

                        //     break 'outer;
                        // }

                        // bitmap.clear(&resting_order_index);
                        // bitmap_group_traversed = true;
                    }
                }
            }
            // Previous inner index is only used for the first active tick
            if previous_inner_index.is_some() {
                previous_inner_index = None;
            }

            // Empty bitmap group written to slot
            bitmap_group.write_to_slot(slot_storage, &outer_index);

            list_slot.clear_index(&list_key);
            list_slot_traversed = true;

            if relative_index == 0 {
                break;
            }
            relative_index -= 1;
        }

        // All orders for the slot index have been purged
        // Empty list slot written to slot
        list_slot.write_to_slot(slot_storage, &list_key);

        if slot_index == 0 {
            break;
        }
        // Move to the next ListSlot. Reset the relative index.
        slot_index -= 1;
        relative_index = 15;
    }
}
