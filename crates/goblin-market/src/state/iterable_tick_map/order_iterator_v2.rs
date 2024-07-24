use crate::{
    program::GoblinError,
    quantities::Ticks,
    state::{MarketState, RestingOrder, Side, SlotStorage},
};

use super::{
    inner_indices, BitmapGroup, InnerIndex, ListKey, ListSlot, OrderId, RestingOrderIndex,
    SlotRestingOrder,
};

pub fn process_resting_orders(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    num_ticks: Ticks,
    side: Side,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
    lambda_function: fn(
        resting_order: &mut SlotRestingOrder,
        num_ticks: Ticks,
        resting_order_price: Ticks,
        side: Side,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) -> LambdaResult,
) -> Result<Option<OrderId>, GoblinError> {
    let mut outer_index_count = market_state.outer_index_length(side);
    let mut price_in_ticks = market_state.best_price(side);
    let mut previous_inner_index = Some(price_in_ticks.inner_index());
    let mut slot_index = (outer_index_count - 1) / 16;
    let mut relative_index = (outer_index_count - 1) % 16;

    // let mut stop_reads: Option<bool> = None;
    let mut lambda_result = LambdaResult::ContinueLoop;

    // 1. Loop through index slots
    loop {
        let list_key = ListKey { index: slot_index };
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

                        if lambda_result != LambdaResult::ContinueLoop {
                            if pending_bitmap_group_write {
                                bitmap_group.write_to_slot(slot_storage, &outer_index);
                            }
                            if pending_list_slot_write {
                                list_slot.write_to_slot(slot_storage, &list_key);
                            }
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return match lambda_result {
                                LambdaResult::ReturnNone => Ok(None),
                                LambdaResult::ReturnOrderId => Ok(Some(order_id)),
                                LambdaResult::ContinueLoop => unreachable!(),
                            };
                        }

                        let mut resting_order =
                            SlotRestingOrder::new_from_slot(slot_storage, order_id);

                        // lambda_result = lambda_function(&mut resting_order);
                        lambda_result = lambda_function(
                            &mut resting_order,
                            num_ticks,
                            price_in_ticks,
                            side,
                            current_block,
                            current_unix_timestamp_in_seconds,
                        );

                        resting_order.write_to_slot(slot_storage, &order_id)?;

                        // The input amount is consumed, exit.
                        // Traversed Bitmap groups and ListSlots have been written already
                        if resting_order.size() != 0 {
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return match lambda_result {
                                LambdaResult::ReturnNone => Ok(None),
                                LambdaResult::ReturnOrderId => Ok(Some(order_id)),
                                LambdaResult::ContinueLoop => unreachable!(),
                            };
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

    Ok(None)
}

#[derive(PartialEq, Eq)]
pub enum LambdaResult {
    ContinueLoop,
    ReturnNone,
    ReturnOrderId,
}

pub fn order_crosses(
    resting_order: &mut SlotRestingOrder,
    num_ticks: Ticks,
    resting_order_price: Ticks,
    side: Side,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
) -> LambdaResult {
    let crosses = match side.opposite() {
        Side::Bid => resting_order_price >= num_ticks,
        Side::Ask => resting_order_price <= num_ticks,
    };

    if !crosses {
        return LambdaResult::ReturnNone;
    }

    if resting_order.expired(current_block, current_unix_timestamp_in_seconds) {
        resting_order.clear_order();
        return LambdaResult::ContinueLoop;
    }

    return LambdaResult::ReturnOrderId;
}
