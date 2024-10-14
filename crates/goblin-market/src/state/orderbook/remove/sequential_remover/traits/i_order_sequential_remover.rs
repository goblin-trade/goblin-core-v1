use crate::state::{order::order_id::OrderId, remove::IGroupPositionRemover, ArbContext};

use super::{
    IGroupPositionSequentialRemover, IOrderSequentialRemoverInner, IOuterIndexSequentialRemover,
};

pub trait IOrderSequentialRemover<'a>: IOrderSequentialRemoverInner<'a> {
    /// Gets the next active order ID and clears the previously returned one.
    ///
    /// There is no need to clear garbage bits since we always begin from
    /// best market price
    fn next(&mut self, ctx: &mut ArbContext) -> Option<OrderId> {
        loop {
            let group_is_uninitialized_or_finished =
                self.group_position_remover().is_uninitialized_or_finished();

            if group_is_uninitialized_or_finished {
                self.outer_index_remover_mut().next(ctx);
            }

            let current_outer_index = self.outer_index();
            match current_outer_index {
                None => return None,
                Some(outer_index) => {
                    if group_is_uninitialized_or_finished {
                        self.group_position_remover_mut()
                            .load_outer_index(ctx, outer_index);
                    }

                    // Find next active group position in group
                    let next_group_position = self.group_position_remover_mut().next();

                    if let Some(group_position) = next_group_position {
                        let order_id = OrderId::from_group_position(group_position, outer_index);
                        let order_price = order_id.price_in_ticks;

                        // Update pending write state
                        let best_price_unchanged = order_price == *self.best_market_price_mut();
                        self.update_pending_write(best_price_unchanged);

                        // Update best market price
                        *self.best_market_price_mut() = order_price;

                        return Some(order_id);
                    }
                }
            };
        }
    }

    /// Concludes the removal. Writes the bitmap group if `pending_write` is true and
    /// updates the outer index count. There are no slot writes involved in the outer
    /// index list for the sequential remover.
    ///
    /// This is the only place in sequential order remover where the bitmap group
    /// can be written to slot.
    ///
    /// Slot writes- bitmap_group only. Market state is updated in memory, where the
    /// best market price and outer index count is updated.
    ///
    /// TODO This function is identical to IOrderLookupRemover::commit(). Can we
    /// have a common interface for both?
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

            self.outer_index_remover_mut().commit();
        }
    }
}
