use crate::{
    program::{GoblinError, GoblinResult, PricesNotInOrder},
    require,
    state::{
        order::{
            order_id::OrderId,
            sorted_order_id::{AskOrderId, BidOrderId},
        },
        remove::order_id_remover::OrderIdRemover,
        MarketState, Side, SlotStorage,
    },
};

/// Boilerplate code to reduce multiple orders for different sides
pub struct RemoveMultipleManager {
    last_bid_order_id: Option<BidOrderId>,
    last_ask_order_id: Option<AskOrderId>,

    removers: [OrderIdRemover; 2],
}

impl RemoveMultipleManager {
    pub fn new(bids_outer_indices: u16, asks_outer_indices: u16) -> Self {
        RemoveMultipleManager {
            last_bid_order_id: None,
            last_ask_order_id: None,
            removers: [
                OrderIdRemover::new(bids_outer_indices, Side::Bid),
                OrderIdRemover::new(asks_outer_indices, Side::Ask),
            ],
        }
    }

    fn remover(&mut self, side: Side) -> &mut OrderIdRemover {
        &mut self.removers[side as usize]
    }

    /// Ensures that successive order ids to remove are sorted in correct order
    ///
    /// Successive IDs must be in ascending order for asks and in descending order for bids
    pub fn check_sorted(&mut self, side: Side, order_id: OrderId) -> GoblinResult<()> {
        match side {
            Side::Bid => {
                let bid_order_id = BidOrderId { inner: order_id };
                if let Some(last_bid_order_id) = self.last_bid_order_id {
                    require!(
                        bid_order_id < last_bid_order_id,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                }
                self.last_bid_order_id = Some(bid_order_id);
            }
            Side::Ask => {
                let ask_order_id = AskOrderId { inner: order_id };
                if let Some(last_ask_order_id) = self.last_ask_order_id {
                    require!(
                        ask_order_id < last_ask_order_id,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                }
                self.last_ask_order_id = Some(ask_order_id);
            }
        }

        Ok(())
    }

    // TODO read order_id from the struct itself
    pub fn order_present(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        order_id: OrderId,
    ) -> bool {
        self.remover(side).find_order(slot_storage, order_id)
    }

    pub fn remove_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
        side: Side,
    ) {
        self.remover(side).remove_order(slot_storage, market_state)
    }

    pub fn write_prepared_indices(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
    ) {
        self.removers[0].write_prepared_indices(slot_storage, market_state);
        self.removers[1].write_prepared_indices(slot_storage, market_state);
    }
}
