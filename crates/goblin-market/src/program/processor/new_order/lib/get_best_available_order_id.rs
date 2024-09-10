use crate::{
    program::OrderToInsert,
    quantities::Ticks,
    state::{
        bitmap_group::{inner_indices, BitmapGroup, MutableBitmap},
        order::order_id::OrderId,
        InnerIndex, OrderPacket, OuterIndex, Side, SlotStorage, TickIndices,
    },
};

/// Find the best available free order ID where a resting order can be placed,
/// at `price` or better (away from centre).
/// Returns None if no space is available for the given number of amendments.
///
/// # Arguments
///
/// * `slot_storage` - Read only mode
/// * `order_packet`
/// * `last_order` - The last order, if placing multiple post-only orders. If order id
/// and expiry params match, then return the same order id as the last order.
///
pub fn get_best_available_order_id(
    slot_storage: &SlotStorage,
    order_packet: &OrderPacket,
    last_order: Option<OrderToInsert>,
) -> Option<OrderId> {
    let price_in_ticks = order_packet.get_price_in_ticks();
    let side = order_packet.side();

    // If the current and last order have the same order ID but different expiry
    // params, then construct a virtual bitmap where bit for the previous order is turned on.
    let mut skip_bit_for_last_order = false;

    if let Some(OrderToInsert {
        order_id,
        resting_order,
    }) = last_order
    {
        if order_id.price_in_ticks == price_in_ticks {
            // If expiry parameters are the same, then return same order id as
            // the previous order so that the two orders can be merged.
            if resting_order.track_block == order_packet.track_block()
                && resting_order.last_valid_block_or_unix_timestamp_in_seconds
                    == order_packet.last_valid_block_or_unix_timestamp_in_seconds()
            {
                return Some(order_id);
            } else {
                skip_bit_for_last_order = true;
            }
        }
    }

    let TickIndices {
        outer_index,
        inner_index,
    } = price_in_ticks.to_indices();

    let mut current_outer_index = outer_index;
    let mut ticks_to_traverse = order_packet.tick_offset();

    // 1. Loop through bitmap groups
    loop {
        let bitmap_group = BitmapGroup::new_from_slot(slot_storage, current_outer_index);

        let previous_inner_index = if current_outer_index == outer_index {
            Some(inner_index)
        } else {
            None
        };

        // 2. Loop through bitmaps
        for i in inner_indices(side, previous_inner_index) {
            let current_inner_index = InnerIndex::new(i);
            let price_in_ticks = Ticks::from_indices(outer_index, current_inner_index);

            // 3. Loop through resting order IDs
            let best_free_index = if skip_bit_for_last_order {
                // Mark as false. This is a one time operation for the first bitmap group
                skip_bit_for_last_order = false;

                // Construct a virtual bitmap which includes activated bit from the last order
                let mut bitmap_raw = bitmap_group.inner[current_inner_index.as_usize()];
                let mut virtual_bitmap = MutableBitmap {
                    inner: &mut bitmap_raw,
                };
                let relative_index_of_last_order = last_order.unwrap().order_id.resting_order_index;
                virtual_bitmap.activate(&relative_index_of_last_order);

                // Lookup from relative_index_of_last_order. This index is filled so it
                // will be skipped.
                virtual_bitmap.best_free_index(relative_index_of_last_order.as_u8())
            } else {
                let bitmap = bitmap_group.get_bitmap(&current_inner_index);

                bitmap.best_free_index(0)
            };

            if let Some(resting_order_index) = best_free_index {
                return Some(OrderId {
                    price_in_ticks,
                    resting_order_index,
                });
            };

            if ticks_to_traverse == 0 {
                return None;
            }
            ticks_to_traverse -= 1;
        }

        if side == Side::Bid {
            if current_outer_index == OuterIndex::ZERO {
                break;
            }
            current_outer_index -= OuterIndex::ONE;
        } else {
            if current_outer_index == OuterIndex::MAX {
                break;
            }
            current_outer_index += OuterIndex::ONE;
        }
    }

    None
}
