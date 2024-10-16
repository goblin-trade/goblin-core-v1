use crate::state::{
    order::{group_position::GroupPosition, order_id::OrderId},
    remove::{IGroupPositionRemover, IOrderSequentialRemover},
    ArbContext,
};

use super::{IGroupPositionLookupRemover, IOrderLookupRemoverInner, IOuterIndexLookupRemover};

pub trait IOrderLookupRemover<'a>: IOrderLookupRemoverInner<'a> {
    /// Lookup the given order ID
    ///
    /// # Arguments
    ///
    /// * `ctx`
    /// * `order_id` - Order to search
    ///
    /// # Returns
    ///
    /// * `true` if the order id is present in the book
    /// * `false` if the order id is not present
    fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        let price = order_id.price_in_ticks;
        let outer_index = price.outer_index();
        let previous_outer_index = self.outer_index();

        if self.pending_write() {
            // previous_outer_index is guaranteed to exist if pending_write is true
            let previous_outer_index = previous_outer_index.unwrap();
            if previous_outer_index != outer_index {
                self.group_position_remover_mut()
                    .write_to_slot(ctx, previous_outer_index);

                *self.pending_write_mut() = false;
            }
        }
        // Prevous outer index is None or not equal to the new outer index
        if previous_outer_index != Some(outer_index) {
            let outer_index_found = self.outer_index_remover_mut().find(ctx, outer_index);
            if !outer_index_found {
                return false;
            }
            self.group_position_remover_mut()
                .load_outer_index(ctx, outer_index);
        }
        self.group_position_remover_mut()
            .find(GroupPosition::from(&order_id))
    }

    /// Remove the last searched order id from the book
    ///
    /// # Arguments
    ///
    /// * `ctx`
    fn remove(&mut self, ctx: &mut ArbContext) {
        if let Some(order_id) = self.order_id() {
            let price = order_id.price_in_ticks;
            let group_position = GroupPosition::from(&order_id);

            // If market price will change on removal, i.e. current order id
            // is the only active bit on best price use the sequential remover
            // to deactivate it and discover the next best market price.
            //
            // Closure of best market price has two subcases
            // * Outermost group closed- sequential remover will decrement
            // outer index count
            // * Outermost group not closed
            if price == *self.best_market_price_inner_mut()
                && self
                    .group_position_remover_mut()
                    .is_only_active_bit_on_tick(group_position)
            {
                self.sequential_order_remover().next(ctx);
            } else {
                // Closure will not change the best market price.
                // This has 3 cases
                // * Removal on the best price but there are other active bits present.
                // * Removal on outermost bitmap group
                // * Removal on an inner bitmap group
                //
                // Group remains active in case 1 and 2, but it can close in
                // case 3. If bitmap group remains active we need to write the pending
                // group to slot. Otherwise we can simply remove its outer index.
                //
                self.group_position_remover_mut().remove();

                let group_is_active = self.group_position_remover_mut().is_group_active();
                self.set_pending_write(group_is_active);
                if !group_is_active {
                    self.outer_index_remover_mut().remove();
                }
            }
        }
    }

    /// Concludes the removal. Writes the bitmap group if `pending_write` is true,
    /// updates the outer index count and writes any pending outer index list slots.
    ///
    ///
    /// Slot writes- bitmap_group and index list. Market state is updated in memory, where the
    /// best market price and outer index count is updated.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    ///
    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write() {
                self.group_position_remover()
                    .write_to_slot(ctx, outer_index);
            }
            self.outer_index_remover_mut().commit(ctx);
        }
    }
}
