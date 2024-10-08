use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, OuterIndex, Side,
    },
};

use super::{
    random_outer_index_remover_v2::{commit_outer_index_remover, find_outer_index},
    sequential_order_remover_v2::SequentialOrderRemoverV2,
};

use alloc::vec::Vec;

pub struct RandomOrderRemoverV2<'a> {
    inner: SequentialOrderRemoverV2<'a>,
    cached_outer_indices: Vec<OuterIndex>,
}

impl<'a> RandomOrderRemoverV2<'a> {
    pub fn new(
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        RandomOrderRemoverV2 {
            inner: SequentialOrderRemoverV2::new(side, best_market_price, outer_index_count),
            cached_outer_indices: Vec::new(),
        }
    }

    pub fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        let price = order_id.price_in_ticks;
        let outer_index = price.outer_index();

        let previous_outer_index = self.inner.outer_index();
        if self.inner.pending_write {
            // previous_outer_index is guaranteed to exist if pending_write is true
            let previous_outer_index = previous_outer_index.unwrap();
            if previous_outer_index != outer_index {
                self.inner
                    .group_position_remover
                    .write_to_slot(ctx, previous_outer_index);
                self.inner.pending_write = false;
            }
        }

        // Prevous outer index is None or not equal to the new outer index
        if previous_outer_index != Some(outer_index) {
            let outer_index_found = find_outer_index(
                ctx,
                &mut self.inner.outer_index_remover,
                &mut self.cached_outer_indices,
                outer_index,
            );

            if !outer_index_found {
                return false;
            }

            self.inner
                .group_position_remover
                .load_outer_index(ctx, outer_index);
        }

        self.inner
            .group_position_remover
            .paginate_and_check_if_active(GroupPosition::from(&order_id))
    }

    pub fn remove(&mut self, ctx: &mut ArbContext) {
        if let Some(order_id) = self.inner.order_id() {
            // If outermost, call sequential remover
            // Else simply remove and update pending state

            let price = order_id.price_in_ticks;
            let group_position = GroupPosition::from(&order_id);

            // Handles two cases
            // * Best market price closed
            // * Subcase- outermost group closed
            if price == *self.inner.best_market_price
                && self
                    .inner
                    .group_position_remover
                    .is_only_active_bit_on_tick(group_position)
            {
                self.inner.next_active_order(ctx);
            } else {
                // Cases
                // * Group is still active
                // * Non-outermost group closed

                self.inner.group_position_remover.deactivate();

                // Don't write bitmap group if
                // - Entire group was closed. We will simply remove the outer index.
                //
                // The outermost group closing is already dealt with by the sequential
                // remove branch. We don't have to worry about garbage bits
                //

                let group_is_active = self.inner.group_position_remover.is_group_active();
                self.set_pending_write(group_is_active);

                if !group_is_active {
                    self.inner.outer_index_remover.remove_loaded_index();
                }
            }
        }
    }

    pub fn commit(&mut self, ctx: &mut ArbContext) {
        if self.inner.pending_write {
            // previous_outer_index is guaranteed to exist if pending_write is true
            let previous_outer_index = self.inner.outer_index().unwrap();
            self.inner
                .group_position_remover
                .write_to_slot(ctx, previous_outer_index);
            self.inner.pending_write = false;
        }

        commit_outer_index_remover(
            ctx,
            &mut self.inner.outer_index_remover,
            &mut self.cached_outer_indices,
        );
    }

    // Getters

    pub fn side(&self) -> Side {
        self.inner.outer_index_remover.side()
    }

    pub fn is_group_active(&self) -> bool {
        self.inner.group_position_remover.is_group_active()
    }

    // Setters

    pub fn set_pending_write(&mut self, non_outermost_group_is_active: bool) {
        self.inner.pending_write = non_outermost_group_is_active;
    }
}
