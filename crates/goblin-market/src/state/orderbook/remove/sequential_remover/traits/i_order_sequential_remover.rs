use crate::{
    quantities::Ticks,
    state::{
        order::order_id::OrderId,
        remove::{IGroupPositionRemover, IOuterIndexRemover},
        ArbContext, OuterIndex,
    },
};

use super::{IGroupPositionSequentialRemover, IOuterIndexSequentialRemover};

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

    /// Concludes the removal. Writes the bitmap group if `pending_write` is true, updates
    /// the outer index count and writes the updated outer indices to slot.
    ///
    /// This is the only place in sequential order remover where the bitmap group
    /// can be written to slot.
    ///
    /// Slot writes- bitmap_group only. Market state is updated in memory, where the
    /// best market price and outer index count is updated.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    ///
    fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if *self.pending_write_mut() {
                self.group_position_remover_mut()
                    .write_to_slot(ctx, outer_index);
            }

            self.outer_index_remover_mut().commit();
        }
    }
}

pub trait IOrderSequentialRemoverInner<'a> {
    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl IGroupPositionSequentialRemover;

    /// Mutable reference to group position remover, to lookup and remove outer indices
    fn group_position_remover_mut(&mut self) -> &mut impl IGroupPositionSequentialRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IOuterIndexSequentialRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexSequentialRemover<'a>;

    /// Reference to best market price for current side from market state
    fn best_market_price_mut(&mut self) -> &mut Ticks;

    /// Whether the bitmap group is pending a write
    fn pending_write_mut(&mut self) -> &mut bool;

    /// Bitmap group must be written if active orders remain on the
    /// best price even after closing the bit, i.e. the best market price
    /// remains unchanged
    fn update_pending_write(&mut self, best_price_unchanged: bool) {
        *self.pending_write_mut() = best_price_unchanged;
    }

    // Getters

    /// The current outer index
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }
}
