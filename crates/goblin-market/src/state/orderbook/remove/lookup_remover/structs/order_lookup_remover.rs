use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup,
        order::{group_position::GroupPosition, order_id::OrderId},
        remove::{
            GroupPositionRemover, IGroupPositionLookupRemover, IGroupPositionRemover,
            IGroupPositionSequentialRemover, IOuterIndexLookupRemover, IOuterIndexRemover,
            IOuterIndexSequentialRemover, NextOrderIterator,
        },
        ArbContext, OuterIndex, Side,
    },
};

use super::OuterIndexLookupRemover;

pub struct OrderLookupRemover<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexLookupRemover<'a>,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemover,

    /// Reference to best market price for current side from market state
    pub best_market_price: &'a mut Ticks,

    /// Whether the bitmap group is pending a read
    pub pending_read: bool,

    /// Whether the bitmap group is pending a write
    pub pending_write: bool,
}

impl<'a> OrderLookupRemover<'a> {
    pub fn new(
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        OrderLookupRemover {
            outer_index_remover: OuterIndexLookupRemover::new(side, outer_index_count),
            group_position_remover: GroupPositionRemover::new(side),
            pending_read: false,
            pending_write: false,
            best_market_price,
        }
    }

    // Getters

    /// The current outer index. If the bitmap group becomes empty on removal
    /// then outer index is removed and this function returns None.
    ///
    /// Incoming order ids should be sorted by outer index, moving away from
    /// the centre. If the cached outer index is None due to its group closing,
    /// the next outer index is read from the bitmap list for comparison.
    pub fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover.current_outer_index()
    }

    /// Group position of the looked up order.
    ///
    /// This value can have 3 scenarios
    /// * Group position corresponds to the order id looked up
    /// * If the outermost value was removed by the sequential remover then
    /// group position will point to the next active bit.
    /// * Group position corresponds to the order id removed, but outer index changes.
    /// This happens when the current outer index cleared and we try to lookup order
    /// from a previous group. A new outer index is loaded for comparion but the
    /// group position does not change.
    fn group_position(&self) -> Option<GroupPosition> {
        self.group_position_remover.current_position()
    }

    fn order_id_to_remove(&self) -> Option<OrderId> {
        let outer_index = self.outer_index()?;
        let group_position = self.group_position()?;

        Some(OrderId::from_group_position(group_position, outer_index))
    }

    pub fn write_bitmap_group(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) {
        debug_assert!(self.pending_write);

        let bitmap_group = self.group_position_remover.bitmap_group_mut();
        bitmap_group.write_to_slot(ctx, &outer_index);

        *self.pending_write_mut() = false;
    }

    /// Paginate to the given order id and check whether it is active.
    ///
    /// # Externally ensure that outer indices move away from the centre,
    /// otherwise the the order cannot be found.
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
    pub fn find(&mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
        let price = order_id.price_in_ticks;
        let outer_index = price.outer_index();
        let previous_outer_index = self.outer_index();

        if previous_outer_index != Some(outer_index) {
            // Write group if outer index changed and pending write is true.
            // If outer index remains same then don't write yet.
            // previous_outer_index is guaranteed to exist if pending_write is true

            if let Some(previous_outer_index) = previous_outer_index {
                if self.pending_write {
                    self.write_bitmap_group(ctx, previous_outer_index);
                }
            }

            let outer_index_found = self.outer_index_remover.find_and_load(ctx, outer_index);
            // pending_write() is always set to false before setting pending_read to true.
            self.pending_read = self.outer_index().is_some();

            if !outer_index_found {
                return false;
            }
        }

        if self.pending_read {
            self.group_position_remover
                .load_outer_index(ctx, outer_index);
            self.pending_read = false;
        }

        self.group_position_remover
            .visit_and_check_if_active(GroupPosition::from(&order_id))
    }

    /// Remove the last searched order id from the book
    ///
    /// # Arguments
    ///
    /// * `ctx`
    pub fn remove(&mut self, ctx: &mut ArbContext) {
        if let Some(order_id) = self.order_id_to_remove() {
            let price = order_id.price_in_ticks;
            let group_position = GroupPosition::from(&order_id);

            // Use the sequential remover if this is the outermost active tick.
            // The sequential remover will paginate to the next active tick and
            // update the best market price.
            //
            // Closure of best market price has two subcases
            // * Outermost group closed- sequential remover will decrement
            // outer index count
            // * Outermost group not closed
            if price == *self.best_market_price
                && self
                    .group_position_remover
                    .bitmap_group_mut()
                    .is_lowest_active_resting_order_on_tick(group_position)
            {
                self.next(ctx);
            } else {
                // Closure will not change the best market price.
                // This has 2 cases
                // * Removing any bit on the outermost group except for the outermost
                // active tick
                // * Removal on an inner bitmap group
                //
                // Group remains active in case 1 but it can close in
                // case 2. If bitmap group remains active we need to write the pending
                // group to slot. Otherwise we can simply remove its outer index.
                //
                self.group_position_remover.deactivate_current();

                let bitmap_group = self.group_position_remover.bitmap_group_mut();

                let group_active = bitmap_group.is_group_active();
                *self.pending_write_mut() = group_active;
                if !group_active {
                    self.outer_index_remover.remove();
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
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index() {
            if self.pending_write {
                self.write_bitmap_group(ctx, outer_index);
            }
            self.outer_index_remover.commit(ctx);
        }
    }

    // Sharing functions

    /// Get the current bitmap group for sharing with the opposite side remover
    pub fn get_shared_bitmap_group(&mut self) -> BitmapGroup {
        debug_assert!(self.outer_index().is_some());
        debug_assert!(self.outer_index().unwrap() == self.best_market_price.outer_index());

        *self.group_position_remover.bitmap_group_mut()
    }

    /// Set the shared bitmap group from the opposite side remover and load the outermost
    /// outer index if necessary.
    pub fn set_shared_bitmap_group(
        &mut self,
        ctx: &mut ArbContext,
        outer_index: OuterIndex,
        shared_bitmap_group: BitmapGroup,
    ) {
        debug_assert!(outer_index == self.best_market_price.outer_index());

        self.outer_index_remover.find_and_load(ctx, outer_index);

        *self.group_position_remover.bitmap_group_mut() = shared_bitmap_group;
    }
}

impl<'a> NextOrderIterator<'a> for OrderLookupRemover<'a> {
    fn group_position_sequential_remover(&mut self) -> &mut impl IGroupPositionSequentialRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexSequentialRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}
