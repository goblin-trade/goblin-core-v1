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
        let best_market_price = self.best_market_price_inner();
        let is_first_read = self.outer_index().is_none();

        loop {
            let group_is_uninitialized_or_finished = self
                .group_position_remover()
                .is_uninitialized_or_exhausted();

            if group_is_uninitialized_or_finished {
                self.outer_index_remover_mut().next(ctx);
            }

            match self.outer_index() {
                Some(outer_index) => {
                    if is_first_read {
                        self.group_position_remover_mut()
                            .load_outermost_group(ctx, best_market_price);
                    } else if group_is_uninitialized_or_finished {
                        // uninitialized case is reached only when next() is called
                        // for the first time.
                        self.group_position_remover_mut()
                            .load_outer_index(ctx, outer_index);
                    }

                    let next_group_position = self.group_position_remover_mut().next();

                    if let Some(next_group_position) = next_group_position {
                        let next_order_id =
                            OrderId::from_group_position(next_group_position, outer_index);
                        let next_order_price = next_order_id.price_in_ticks;

                        let best_price_unchanged = next_order_price == best_market_price;
                        self.update_pending_write(is_first_read, best_price_unchanged);

                        // Update best market price
                        *self.best_market_price_inner_mut() = next_order_price;

                        return Some(next_order_id);
                    }
                }
                None => {
                    // All active bits are exhausted
                    // Set pending write to false so that the group position is not written
                    self.update_pending_write(false, false);

                    return None;
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
    /// TODO This function is similar to IOrderLookupRemover::commit(). Can we
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

            // difference- ctx not passed to commit()
            self.outer_index_remover_mut().commit();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::Ticks,
        state::{
            bitmap_group::BitmapGroup,
            remove::{IOuterIndexRemover, OrderSequentialRemover},
            ContextActions, InnerIndex, ListKey, ListSlot, OuterIndex, RestingOrderIndex, Side,
        },
    };

    use super::IOrderSequentialRemover;

    use super::*;

    #[test]
    fn test_read_asks_across_groups() {
        let ctx = &mut ArbContext::new();
        let side = Side::Ask;

        let outer_index_0 = OuterIndex::new(1);
        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
        bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
        bitmap_group_0.inner[31] = 0b0000_0001;
        bitmap_group_0.write_to_slot(ctx, &outer_index_0);

        let outer_index_1 = OuterIndex::new(2);
        let mut bitmap_group_1 = BitmapGroup::default();
        bitmap_group_1.inner[0] = 0b0000_0001;
        bitmap_group_1.write_to_slot(ctx, &outer_index_1);

        let outer_index_2 = OuterIndex::new(5);
        let mut bitmap_group_2 = BitmapGroup::default();
        bitmap_group_2.inner[0] = 0b0000_0001;
        bitmap_group_2.write_to_slot(ctx, &outer_index_2);

        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_2);
        list_slot.set(1, outer_index_1);
        list_slot.set(2, outer_index_0);
        list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

        let mut outer_index_count = 3;
        let mut best_ask_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));

        let mut remover =
            OrderSequentialRemover::new(side, &mut best_ask_price, &mut outer_index_count);

        // Read the first value- garbage bit is ignored
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        // Read the next value and clear previous
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(2)
            }
        );
        // pending write is true because the previous value was cleared yet best
        // market price did not close
        assert_eq!(remover.pending_write, true);

        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(31)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        // All active bits on current group exhausted, move to next group
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        // Move to final group
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_2, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        assert_eq!(
            remover
                .outer_index_remover
                .active_outer_index_iterator
                .outer_index_count(),
            0
        );
        assert_eq!(remover.outer_index().unwrap(), outer_index_2);

        assert_eq!(remover.next(ctx), None);
        assert_eq!(remover.pending_write, false);
        assert_eq!(remover.outer_index(), None);

        // Best market price does not change when the last active bit is closed
        assert_eq!(
            remover.best_market_price_inner(),
            Ticks::from_indices(outer_index_2, InnerIndex::new(0))
        );
    }

    #[test]
    fn test_read_bids_across_groups() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;

        let outer_index_0 = OuterIndex::new(5);
        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[0] = 0b1000_0000;
        bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
        bitmap_group_0.inner[31] = 0b0000_0001; // Garbage bit
        bitmap_group_0.write_to_slot(ctx, &outer_index_0);

        let outer_index_1 = OuterIndex::new(2);
        let mut bitmap_group_1 = BitmapGroup::default();
        bitmap_group_1.inner[31] = 0b0000_0001;
        bitmap_group_1.write_to_slot(ctx, &outer_index_1);

        let outer_index_2 = OuterIndex::new(1);
        let mut bitmap_group_2 = BitmapGroup::default();
        bitmap_group_2.inner[0] = 0b0000_0001;
        bitmap_group_2.write_to_slot(ctx, &outer_index_2);

        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_2);
        list_slot.set(1, outer_index_1);
        list_slot.set(2, outer_index_0);
        list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

        let mut outer_index_count = 3;
        let mut best_ask_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));

        let mut remover =
            OrderSequentialRemover::new(side, &mut best_ask_price, &mut outer_index_count);

        // Read the first value- garbage bit is ignored
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        // assert_eq!(remover.pending_write, false);

        // Read the next value and clear previous
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(1)),
                resting_order_index: RestingOrderIndex::new(2)
            }
        );
        // pending write is true because the previous value was cleared yet best
        // market price did not close
        assert_eq!(remover.pending_write, true);

        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_0, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(7)
            }
        );
        assert_eq!(remover.pending_write, false);

        // All active bits on current group exhausted, move to next group
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_1, InnerIndex::new(31)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        // Move to final group
        assert_eq!(
            remover.next(ctx).unwrap(),
            OrderId {
                price_in_ticks: Ticks::from_indices(outer_index_2, InnerIndex::new(0)),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.pending_write, false);

        assert_eq!(
            remover
                .outer_index_remover
                .active_outer_index_iterator
                .outer_index_count(),
            0
        );
        assert_eq!(remover.outer_index().unwrap(), outer_index_2);

        assert_eq!(remover.next(ctx), None);
        assert_eq!(remover.pending_write, false);
        assert_eq!(remover.outer_index(), None);

        // Best market price does not change when the last active bit is closed
        assert_eq!(
            remover.best_market_price_inner(),
            Ticks::from_indices(outer_index_2, InnerIndex::new(0))
        );
    }

    #[test]
    fn test_commit() {
        let ctx = &mut ArbContext::new();
        let side = Side::Ask;

        let outer_index_0 = OuterIndex::new(1);
        let mut bitmap_group_0 = BitmapGroup::default();
        bitmap_group_0.inner[0] = 0b1000_0000; // Garbage bit
        bitmap_group_0.inner[1] = 0b0000_0101; // Best market price starts here
        bitmap_group_0.inner[31] = 0b0000_0001;
        bitmap_group_0.write_to_slot(ctx, &outer_index_0);

        let outer_index_1 = OuterIndex::new(2);
        let mut bitmap_group_1 = BitmapGroup::default();
        bitmap_group_1.inner[0] = 0b0000_0001;
        bitmap_group_1.write_to_slot(ctx, &outer_index_1);

        let mut list_slot = ListSlot::default();
        list_slot.set(0, outer_index_1);
        list_slot.set(1, outer_index_0);
        list_slot.write_to_slot(ctx, &ListKey { index: 0, side });

        let mut outer_index_count = 2;
        let mut best_ask_price = Ticks::from_indices(outer_index_0, InnerIndex::new(1));

        let mut remover =
            OrderSequentialRemover::new(side, &mut best_ask_price, &mut outer_index_count);

        remover.next(ctx);

        assert!(remover.outer_index().is_some());
        assert_eq!(
            remover
                .outer_index_remover()
                .active_outer_index_iterator()
                .outer_index_count(),
            1
        );

        remover.commit(ctx);
        assert_eq!(outer_index_count, 2);
    }

    // TODO random input tests with quickcheck
}
