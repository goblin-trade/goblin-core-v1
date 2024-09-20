use crate::{
    program::{GoblinError, GoblinResult, PricesNotInOrder},
    require,
    state::{
        order::{order_id::OrderId, sorted_order_id::orders_are_sorted},
        remove::order_id_remover::OrderIdRemover,
        MarketState, Side, SlotStorage,
    },
};

/// Boilerplate code to reduce multiple orders in bulk for both sides
///
pub struct RemoveMultipleManager {
    side: Side,
    last_order_ids: [Option<OrderId>; 2],
    removers: [OrderIdRemover; 2],
}

impl RemoveMultipleManager {
    pub fn new(bids_outer_indices: u16, asks_outer_indices: u16) -> Self {
        RemoveMultipleManager {
            side: Side::Bid,
            last_order_ids: [None, None],
            removers: [
                OrderIdRemover::new(bids_outer_indices, Side::Bid),
                OrderIdRemover::new(asks_outer_indices, Side::Ask),
            ],
        }
    }

    fn remover(&mut self) -> &mut OrderIdRemover {
        &mut self.removers[self.side as usize]
    }

    fn last_order_id(&mut self) -> &mut Option<OrderId> {
        &mut self.last_order_ids[self.side as usize]
    }

    /// Checks whether an order is present at the given order ID.
    pub fn find_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        order_id: OrderId,
    ) -> GoblinResult<bool> {
        self.check_sorted(side, order_id)?;

        let found = self.remover().find_order(slot_storage, order_id);
        Ok(found)
    }

    /// Ensures that successive order ids to remove are sorted in correct order
    ///
    /// Successive IDs must be in ascending order for asks and in descending order for bids
    pub(crate) fn check_sorted(&mut self, side: Side, order_id: OrderId) -> GoblinResult<()> {
        self.side = side;
        let last_order_id = self.last_order_id();

        // Successive orders must move away from the centre
        if let Some(last_order_id) = *last_order_id {
            let sorted = orders_are_sorted(side, order_id, last_order_id);
            require!(sorted, GoblinError::PricesNotInOrder(PricesNotInOrder {}));
        }
        // Set as last order ID
        *last_order_id = Some(order_id);

        Ok(())
    }

    /// Remove the last searched order from the book, and update the
    /// best price in market state if the outermost tick closed
    pub fn remove_order(&mut self, slot_storage: &mut SlotStorage, market_state: &mut MarketState) {
        self.remover().remove_order(slot_storage, market_state)
    }

    /// Write the prepared outer indices to slot and update outer index count in market state
    /// The last cached bitmap group pending a write is also written to slot
    pub fn write_prepared_indices(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
    ) {
        self.removers[0].write_prepared_indices(slot_storage, market_state);
        self.removers[1].write_prepared_indices(slot_storage, market_state);
    }
}
