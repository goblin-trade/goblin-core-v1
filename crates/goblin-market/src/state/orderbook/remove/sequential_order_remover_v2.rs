use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, OuterIndex, Side,
    },
};

use super::{
    group_position_remover_v2::{GroupPositionRemoverV2, SequentialGroupPositionRemover},
    sequential_outer_index_remover::SequentialOuterIndexRemover,
};

pub struct SequentialOrderRemoverV2<'a> {
    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemoverV2,

    pub outer_index_remover: SequentialOuterIndexRemover<'a>,

    pub best_market_price: &'a mut Ticks,

    /// Whether the bitmap group is pending a write
    pub pending_write: bool,
}

impl<'a> SequentialOrderRemoverV2<'a> {
    pub fn new(
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        SequentialOrderRemoverV2 {
            group_position_remover: GroupPositionRemoverV2::new(side),
            outer_index_remover: SequentialOuterIndexRemover::new(side, outer_index_count),
            pending_write: false,
            best_market_price,
        }
    }

    /// Gets the next active order ID and clears the previously returned one.
    ///
    /// There is no need to clear garbage bits since we always begin from
    /// best market price
    pub fn next_active_order(&mut self, ctx: &mut ArbContext) -> Option<OrderId> {
        loop {
            // Check if outer index is loaded
            let outer_index = self.outer_index_remover.get_or_load_outer_index(ctx);

            match outer_index {
                None => return None,
                Some(outer_index) => {
                    if self.group_position_remover.is_uninitialized_or_finished() {
                        self.group_position_remover
                            .load_outer_index(ctx, outer_index);
                    }

                    // Find next active group position in group
                    let next_group_position = self
                        .group_position_remover
                        .deactivate_current_and_get_next();

                    match next_group_position {
                        Some(group_position) => {
                            let order_id =
                                OrderId::from_group_position(group_position, outer_index);
                            let order_price = order_id.price_in_ticks;

                            // Update pending write state
                            let best_price_unchanged = order_price == *self.best_market_price;
                            self.update_pending_write(best_price_unchanged);

                            // Update best market price
                            *self.best_market_price = order_price;

                            return Some(order_id);
                        }
                        None => {
                            // This will clear outer index
                            // Can we avoid this call?
                            // * Instead of calling get_or_load_outer_index() in the top,
                            // call load_and_return_outer_index(). This will clear the previous
                            // value, load the new one and return it.
                            //
                            // However outer index must be persisted across loops.
                            // Can we have an inner loop.
                            //
                            // Suppose outer index 0 has two positions in its group.
                            // 1. First call: return position 1.
                            // 2. Second call: return position 2.
                            // The outer index was retained.
                            //
                            // Solution- check if group position is present at the start.
                            // If not then load new outer index.
                            self.outer_index_remover.remove_loaded_index();
                        }
                    };
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
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write {
                self.group_position_remover.write_to_slot(ctx, outer_index);
            }

            self.outer_index_remover.commit();
        }
    }

    /// Bitmap group must be written if active orders remain on the
    /// best price even after closing the bit, i.e. the best market price
    /// remains unchanged
    fn update_pending_write(&mut self, best_price_unchanged: bool) {
        self.pending_write = best_price_unchanged;
    }

    pub fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover.current_outer_index
    }

    pub fn group_position(&self) -> Option<GroupPosition> {
        self.group_position_remover.group_position()
    }

    pub fn order_id(&self) -> Option<OrderId> {
        let outer_index = self.outer_index()?;
        let group_position = self.group_position()?;

        Some(OrderId::from_group_position(group_position, outer_index))
    }
}
