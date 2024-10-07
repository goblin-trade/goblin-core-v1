use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, InnerIndex, MarketPrices, OuterIndex, RestingOrderIndex, Side,
    },
};

use super::{
    group_position_remover_v2::GroupPositionRemoverV2, outer_index_remover::OuterIndexRemover,
};

pub struct SequentialOrderRemover<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexRemover,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemoverV2,

    /// Whether the bitmap group was updated in memory and is pending a write.
    /// write_last_bitmap_group() should write to slot only if this is true.
    pub pending_write: bool,

    pub best_market_price: &'a mut Ticks,

    pub best_opposite_price: Ticks,

    pub outer_index_count: &'a mut u16,
}

impl<'a> SequentialOrderRemover<'a> {
    pub fn new(
        outer_index_count: &'a mut u16,
        side: Side,
        best_market_price: &'a mut Ticks,
        best_opposite_price: Ticks,
    ) -> Self {
        SequentialOrderRemover {
            group_position_remover: GroupPositionRemoverV2::new(side),
            outer_index_remover: OuterIndexRemover::new(side, *outer_index_count),
            pending_write: false,
            best_market_price,
            best_opposite_price,
            outer_index_count,
        }
    }

    /// Gets the next active order ID and clears the previously returned one.
    ///
    /// There is no need to clear garbage bits since we always begin from
    /// best market price
    pub fn next_active_order(&mut self, ctx: &mut ArbContext) -> Option<OrderId> {
        loop {
            // Check if outer index is loaded
            let outer_index = self.outer_index_remover.get_outer_index(ctx);

            match outer_index {
                None => return None,
                Some(outer_index) => {
                    // Do we need to load bitmap group, or is it already present?
                    if self.group_position_remover.is_uninitialized_or_finished() {
                        self.group_position_remover
                            .load_outer_index(ctx, outer_index);
                    }

                    // Find next active group position in group
                    let group_position = self.group_position_remover.clear_previous_and_get_next();

                    match group_position {
                        Some(group_position) => {
                            let order_id =
                                OrderId::from_group_position(group_position, outer_index);

                            self.update_pending_state_and_best_price(order_id.price_in_ticks);
                            return Some(order_id);
                        }
                        None => {
                            self.outer_index_remover.remove_cached_index();
                        }
                    };
                }
            };
        }
    }

    /// Concludes the removal. Writes the bitmap group if `pending_write` is true, updates
    /// the outer index count and writes the updated outer indices to slot.
    ///
    /// In sequential remove case-
    /// The bitmap group is written only if the last removal does not update the best market price.
    ///
    /// Slot writes- bitmap_group, index list. Market state is only updated in memory.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    ///
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        self.write_bitmap_group(ctx);
        *self.outer_index_count = self.outer_index_remover.index_list_length();
        self.outer_index_remover.write_index_list(ctx);
    }

    // Setters

    /// Updates pending write state and best market price if the next active
    /// order has a different price
    fn update_pending_state_and_best_price(&mut self, new_price: Ticks) {
        let best_price_closed = new_price != *self.best_market_price;
        self.update_pending_write_on_sequential_remove(best_price_closed);

        if best_price_closed {
            *self.best_market_price = new_price;
        }
    }

    /// Sets pending write to true if the best tick does not close, and false if otherwise
    pub fn update_pending_write_on_sequential_remove(&mut self, best_price_closed: bool) {
        self.pending_write = !best_price_closed
    }

    /// Writes bitmap group to slot if a write is pending
    pub fn write_bitmap_group(&mut self, ctx: &mut ArbContext) {
        if !self.pending_write {
            return;
        }
        // If pending_write is true then outer_index is guaranteed to be present
        let outer_index = self.outer_index_unchecked();

        self.group_position_remover
            .inner
            .bitmap_group
            .write_to_slot(ctx, &outer_index);
        self.pending_write = false;
    }

    /// TODO move outside
    pub fn update_pending_write(&mut self, best_price_closed: bool, bitmap_group_closed: bool) {
        self.pending_write = !(best_price_closed || bitmap_group_closed)
    }

    // Getters

    pub fn side(&self) -> Side {
        self.group_position_remover.side()
    }

    pub fn last_outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover.cached_outer_index
    }

    pub fn market_prices(&self) -> MarketPrices {
        match self.side() {
            Side::Bid => MarketPrices {
                best_bid_price: *self.best_market_price,
                best_ask_price: self.best_opposite_price,
            },
            Side::Ask => MarketPrices {
                best_bid_price: self.best_opposite_price,
                best_ask_price: *self.best_market_price,
            },
        }
    }

    // Unsafe getters

    // Externally ensure that order index is present
    pub fn outer_index_unchecked(&self) -> OuterIndex {
        let outer_index = self.last_outer_index();
        debug_assert!(outer_index.is_some());

        unsafe { outer_index.unwrap_unchecked() }
    }

    pub fn group_position_unchecked(&self) -> GroupPosition {
        self.group_position_remover.group_position_unchecked()
    }

    pub fn inner_index_unchecked(&self) -> InnerIndex {
        self.group_position_unchecked().inner_index
    }

    pub fn resting_order_index_unchecked(&self) -> RestingOrderIndex {
        self.group_position_unchecked().resting_order_index
    }

    pub fn price_unchecked(&self) -> Ticks {
        Ticks::from_indices(self.outer_index_unchecked(), self.inner_index_unchecked())
    }

    pub fn order_id_unchecked(&self) -> OrderId {
        OrderId::from_group_position(
            self.group_position_unchecked(),
            self.outer_index_unchecked(),
        )
    }
}
