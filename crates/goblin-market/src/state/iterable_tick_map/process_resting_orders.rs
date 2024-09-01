use crate::{
    quantities::Ticks,
    state::{
        InnerIndex, MarketState, OrderId, RestingOrder, RestingOrderIndex, Side, SlotRestingOrder,
        SlotStorage,
    },
};

use super::{inner_indices, BitmapGroup, ListKey, ListSlot};

/// Loops through subsequent resting orders, applying a lambda function on each.
///
/// - Traversed resting orders are removed from bitmap groups and the index list.
/// The market state is updated accordingly.
/// - Looping stops if lambda function returns true.
///
pub fn process_resting_orders(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    side: Side,
    lambda: &mut dyn FnMut(OrderId, &mut SlotRestingOrder, &mut SlotStorage) -> bool,
) {
    let mut price_in_ticks = market_state.best_price(side);
    let mut previous_inner_index = Some(price_in_ticks.inner_index());

    let mut outer_index_count = market_state.outer_index_count(side);
    let mut slot_index = (outer_index_count - 1) / 16;
    let mut relative_index = (outer_index_count - 1) % 16;
    let mut stop = false;

    // 1. Loop through index slots
    loop {
        let list_key = ListKey {
            index: slot_index,
            side,
        };
        let mut list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut pending_list_slot_write = false;

        // 2. Loop through bitmap groups using relative index
        loop {
            let outer_index = list_slot.get(relative_index as usize);
            let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            let mut pending_bitmap_group_write = false;

            // 3. Loop through bitmaps
            for i in inner_indices(side, previous_inner_index) {
                let inner_index = InnerIndex::new(i);
                price_in_ticks = Ticks::from_indices(outer_index, inner_index);
                let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);

                // 4. Loop through resting orders in the bitmap
                for j in 0..8 {
                    let resting_order_index = RestingOrderIndex::new(j);
                    let order_present = bitmap.order_present(resting_order_index);

                    if order_present {
                        let order_id = OrderId {
                            price_in_ticks,
                            resting_order_index,
                        };

                        if stop {
                            if pending_bitmap_group_write {
                                bitmap_group.write_to_slot(slot_storage, &outer_index);
                            }
                            if pending_list_slot_write {
                                list_slot.write_to_slot(slot_storage, &list_key);
                            }
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return;
                        }

                        let mut resting_order =
                            SlotRestingOrder::new_from_slot(slot_storage, order_id);

                        stop = lambda(order_id, &mut resting_order, slot_storage);

                        resting_order
                            .write_to_slot(slot_storage, &order_id)
                            .unwrap();

                        if stop && resting_order.size() != 0 {
                            if pending_bitmap_group_write {
                                bitmap_group.write_to_slot(slot_storage, &outer_index);
                            }
                            if pending_list_slot_write {
                                list_slot.write_to_slot(slot_storage, &list_key);
                            }
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return;
                        }

                        bitmap.clear(&resting_order_index);
                        pending_bitmap_group_write = true;
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
            pending_list_slot_write = true;
            outer_index_count -= 1;

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
