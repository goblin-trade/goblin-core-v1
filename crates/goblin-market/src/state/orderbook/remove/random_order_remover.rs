use crate::{
    quantities::Ticks,
    state::{order::order_id::OrderId, ArbContext, OuterIndex, Side, TickIndices},
};

use super::sequential_order_remover::SequentialOrderRemover;

pub struct RandomOrderRemover<'a> {
    inner: SequentialOrderRemover<'a>,
}

impl<'a> RandomOrderRemover<'a> {
    pub fn new(
        outer_index_count: &'a mut u16,
        side: Side,
        best_market_price: &'a mut Ticks,
        best_opposite_price: Ticks,
    ) -> Self {
        RandomOrderRemover {
            inner: SequentialOrderRemover::new(
                outer_index_count,
                side,
                best_market_price,
                best_opposite_price,
            ),
        }
    }

    pub fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        let OrderId {
            price_in_ticks,
            resting_order_index,
        } = order_id;
        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        // 1. Navigate to the outer index if not already
        // 2. Load bitmap group and flush garbage bits
        // 3.

        // Load outer index
        // Read the bitmap group and outer index corresponding to order_id
        if self.last_outer_index() != Some(outer_index) {}

        // if self.last_outer_index() != Some(outer_index) {
        //     let outer_index_present = self.try_load_outer_index(ctx, outer_index);
        //     if !outer_index_present {
        //         return false;
        //     }

        //     // self.try_clear_garbage_bits(market_prices);
        // }
        false
    }

    // pub fn remove(&mut self) {
    //     // If the outermost is being removed, call SequentialOrderRemover::remove_inner()

    //     let outermost_removed = false;

    //     if outermost_removed {
    //         // Best market price may or may not close
    //         // Call SequentialOrderRemover::next_active_order()
    //         // This will clear the current order, move to the next active order
    //         // and perform market price update
    //     } else {
    //         // Remove as usual
    //         // Best market price does not close
    //         // Check whether the group closes- group can close only in a non-outermost
    //         // bitmap group
    //     }
    // }

    pub fn last_outer_index(&self) -> Option<OuterIndex> {
        self.inner.last_outer_index()
    }
}
