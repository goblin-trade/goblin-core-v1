use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{BaseLots, QuoteLots},
};

use super::{
    trader_state, BitmapGroup, MarketState, MatchingEngineResponse, OrderId, RestingOrder, Side,
    SlotActions, SlotRestingOrder, SlotStorage, TraderState,
};
use alloc::vec::Vec;

pub struct MatchingEngine<'a> {
    pub slot_storage: &'a mut SlotStorage,
}

impl MatchingEngine<'_> {
    pub fn collect_fees(&mut self) -> QuoteLots {
        // Read
        let mut market = MarketState::read_from_slot(self.slot_storage);

        // Mutate
        let quote_lot_fees = market.unclaimed_quote_lot_fees;

        // Mark as claimed
        market.collected_quote_lot_fees += market.unclaimed_quote_lot_fees;
        market.unclaimed_quote_lot_fees = QuoteLots::ZERO;

        // Write
        market.write_to_slot(self.slot_storage);
        SlotStorage::storage_flush_cache(true);

        quote_lot_fees
    }

    /// Try to claim the given number of lots from a trader's state.
    ///
    /// There is no eviction in Goblin.
    ///
    /// # Parameters
    ///
    /// * `trader` - The trader address
    /// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    /// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U32::MAX to withdraw all. (max value of base_lots is U32::MAX)
    ///
    pub fn claim_funds(
        &mut self,
        trader: Address,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> MatchingEngineResponse {
        // Read
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        // Mutate
        let response = trader_state.claim_funds_inner(num_quote_lots, num_base_lots);

        // Write
        trader_state.write_to_slot(self.slot_storage, trader);
        SlotStorage::storage_flush_cache(true);

        response
    }

    pub fn reduce_order_inner(
        &mut self,
        trader_state: &mut TraderState,
        trader: Address,
        order_id: &[u8; 32],
        side: Side,
        size: Option<BaseLots>,
        order_is_expired: bool,
        claim_funds: bool,
    ) -> Option<ReduceOrderInnerResponse> {
        let mut order = SlotRestingOrder::new_from_raw_key(self.slot_storage, order_id);

        let mut remove_order = false;

        let removed_base_lots = {
            // whether to remove order completely (clear slot), and lots to remove
            let (should_remove_order_from_book, base_lots_to_remove) = {
                // Empty slot- order doesn't exist
                if order.does_not_exist() {
                    return Some(ReduceOrderInnerResponse::default());
                }

                if order.trader_address != trader {
                    return None;
                }

                let base_lots_to_remove = size
                    .map(|s| s.min(order.num_base_lots))
                    .unwrap_or(order.num_base_lots);

                // If the order is tagged as expired, we remove it from the book regardless of the size.
                if order_is_expired {
                    (true, order.num_base_lots)
                } else {
                    (
                        base_lots_to_remove == order.num_base_lots,
                        base_lots_to_remove,
                    )
                }
            };

            let _base_lots_remaining = if should_remove_order_from_book {
                order.clear_order();

                remove_order = true;

                // update bitmap and index_list externally
                // mutable_bitmap.flip(&order_id.resting_order_index);

                BaseLots::ZERO
            } else {
                // Reduce order
                order.num_base_lots -= base_lots_to_remove;
                order.num_base_lots
            };

            // EMIT ExpiredOrder / Reduce

            base_lots_to_remove
        };
        let order_id_decoded = OrderId::decode(order_id);

        // Store order state
        order
            .write_to_slot(self.slot_storage, &order_id_decoded)
            .ok();

        // We don't want to claim funds if an order is removed from the book during a self trade
        // or if the user specifically indicates that they don't want to claim funds.
        if claim_funds {
            // Update trader state
            let (num_quote_lots, num_base_lots) = {
                match side {
                    Side::Bid => {
                        let quote_lots = (order_id_decoded.price_in_ticks
                            * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                            * removed_base_lots)
                            / BASE_LOTS_PER_BASE_UNIT;
                        trader_state.unlock_quote_lots(quote_lots);

                        (quote_lots, BaseLots::ZERO)
                    }
                    Side::Ask => {
                        trader_state.unlock_base_lots(removed_base_lots);

                        (QuoteLots::ZERO, removed_base_lots)
                    }
                }
            };

            Some(ReduceOrderInnerResponse {
                // TODO externally write trader_state to slot
                matching_engine_response: trader_state
                    .claim_funds_inner(num_quote_lots, num_base_lots),
                remove_order,
            })
        } else {
            Some(ReduceOrderInnerResponse {
                matching_engine_response: MatchingEngineResponse::default(),
                remove_order,
            })
        }
    }

    pub fn cancel_multiple_orders_by_id_inner(
        &mut self,
        trader: Address,
        orders_to_cancel: Vec<B256>,
        claim_funds: bool,
    ) {
        // Call reduce_order_inner() for each order ID. Set size = None to empty the orders

        // Read
        let mut market = MarketState::read_from_slot(self.slot_storage);
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        let mut bid_indices_to_remove = Vec::<u16>::new();
        let mut ask_indices_to_remove = Vec::<u16>::new();

        // Pass order IDs grouped by outer indices for efficient use of cache
        let mut cached_bitmap_group: Option<BitmapGroup> = None;

        // shared between orders- trader_state
        // Do not pass bitmaps or index_list. We only need to update them if order is closed
        // return a `closed: bool` to track this
        for order_id_bytes in orders_to_cancel {
            // - Compare with best_bid_price and best_ask_price to know side
            // - Behavior when one of the order is closed / belongs to a different trader?
            // Unlike Phoenix, order IDs are reused.
            // In phoenix- try to cancel other orders. Ignore failed cancelations (when orders).
            // Since we don't have a way to cancel all orders, traders must lookup their addresses
            // client side then attempt to cancel.
            // `revert_if_fail` field for each order- revert TX if cancel fails (closed order, or belonging to other trader)
            // - How to structure order IDs to optimize gas?

            let order_id = OrderId::decode(&order_id_bytes);
            let side = order_id.side(market.best_bid_price, market.best_ask_price);
            let order = SlotRestingOrder::new_from_slot(self.slot_storage, &order_id);

            // Call reduce_order_inner()

            // let order = SlotRestingOrder::new_from_raw_key(self.slot_storage, &order_id_bytes.0);
            // let resp = self.reduce_order_inner(
            //     &mut trader_state,
            //     trader,
            //     &order_id_bytes.0,
            //     side,
            //     None,
            //     false,
            //     claim_funds,
            // );

            // If order was closed- look at matching_engine_resp and new size
            // - read and update bitmap
            // - if bitmap group becomes 0, queue outer index for removal
            // - ensure that queued indices are in ascending / descending order
            //  This forces the trader to pass them proper order for gas efficiency
            if order.size() == 0 {}
        }

        // use array of queued indices. Remove these from index_list
        // update market state
    }
}

#[derive(Default)]
pub struct ReduceOrderInnerResponse {
    pub matching_engine_response: MatchingEngineResponse,
    pub remove_order: bool,
}
