use crate::{
    quantities::{BaseLots, Ticks},
    state::{
        order::{order_id::OrderId, resting_order::SlotRestingOrder},
        process_resting_orders::process_resting_orders,
        ArbContext, MarketState, Side, TraderState,
    },
};

/// This function determines whether a PostOnly order crosses the book.
/// If the order crosses the book, the function returns the price of the best unexpired
/// crossing order (price, index) on the opposite side of the book. Otherwise, it returns None.
///
/// The function closes all expired orders till an unexpired order is found.
///
/// # Arguments
///
/// * `market_state`
/// * `side`
/// * `num_ticks`
/// * `current_block`
/// * `current_unix_timestamp_in_seconds`
///
pub fn check_for_cross(
    ctx: &mut ArbContext,
    market_state: &mut MarketState,
    side: Side,
    limit_price_in_ticks: Ticks,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
) -> Option<Ticks> {
    let opposite_side = side.opposite();
    let opposite_best_price = market_state.best_price(opposite_side);
    let outer_index_count = market_state.outer_index_count(opposite_side);

    if outer_index_count == 0 // Book empty case
            // No cross case
            || (side == Side::Bid && limit_price_in_ticks < opposite_best_price)
            || (side == Side::Ask && limit_price_in_ticks > opposite_best_price)
    {
        return None;
    }

    let mut crossing_tick: Option<Ticks> = None;

    let mut handle_cross = |order_id: OrderId,
                            resting_order: &mut SlotRestingOrder,
                            ctx: &mut ArbContext| {
        let crosses = match opposite_side {
            Side::Bid => order_id.price_in_ticks >= limit_price_in_ticks,
            Side::Ask => order_id.price_in_ticks <= limit_price_in_ticks,
        };

        if !crosses {
            return true;
        }

        if resting_order.is_expired(current_block, current_unix_timestamp_in_seconds) {
            let mut maker_state = TraderState::read_from_slot(ctx, resting_order.trader_address);

            resting_order.reduce_order(
                &mut maker_state,
                opposite_side,
                order_id.price_in_ticks,
                BaseLots::MAX,
                true,
                false,
            );

            maker_state.write_to_slot(ctx, resting_order.trader_address);

            return false;
        }

        crossing_tick = Some(order_id.price_in_ticks);
        return true;
    };
    process_resting_orders(ctx, market_state, opposite_side, &mut handle_cross);

    crossing_tick
}
