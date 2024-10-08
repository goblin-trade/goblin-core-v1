use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        write_index_list::write_index_list,
        ArbContext, OuterIndex, Side,
    },
};

use super::{
    sequential_order_remover_v2::SequentialOrderRemoverV2,
    sequential_outer_index_remover::SequentialOuterIndexRemover,
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

    // fn random_outer_index_remover(&'a mut self) -> RandomOuterIndexRemover {
    //     RandomOuterIndexRemover {
    //         inner: &mut self.inner.outer_index_remover,
    //         cached_outer_indices: &mut self.cached_outer_indices,
    //     }
    // }

    pub fn find(&'a mut self, ctx: &mut ArbContext, order_id: OrderId) -> bool {
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
            let outer_index_found = self.find_outer_index(ctx, outer_index);

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
                    self.inner.outer_index_remover.remove_cached_index();
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

        self.commit_outer_index_remover(ctx);
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

    // Random outer index remover

    pub fn find_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        // loop till the given outer index is found
        // - write the found value to cached_outer_index
        // - write other values to cache
        //
        // return true if found, false if the list concludes

        let side = self.side();
        let remover = &mut self.inner.outer_index_remover;

        loop {
            if let Some(read_outer_index) = remover.active_outer_index_iterator.next(ctx) {
                if read_outer_index == outer_index {
                    remover.cached_outer_index = Some(read_outer_index);
                    return true;
                } else if side == Side::Bid && read_outer_index > outer_index
                    || side == Side::Ask && read_outer_index < outer_index
                {
                    self.cached_outer_indices.push(read_outer_index);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    pub fn commit_outer_index_remover(&mut self, ctx: &mut ArbContext) {
        // If cached outer index exists, increment the outer index count. No
        // need to push this value to the cached list. This is because the
        // cached outer index is the current outermost value in the index list.
        let unread_count = *self
            .inner
            .outer_index_remover
            .active_outer_index_iterator
            .inner
            .outer_index_count
            + u16::from(self.inner.outer_index().is_some());

        // TODO simplify. write_index_list() is only used here so we
        // can use its code
        write_index_list(
            ctx,
            self.side(),
            &mut self.cached_outer_indices,
            unread_count,
            self.inner
                .outer_index_remover
                .active_outer_index_iterator
                .list_slot,
        );
    }
}

pub struct RandomOuterIndexRemover<'a> {
    inner: &'a mut SequentialOuterIndexRemover<'a>,
    cached_outer_indices: &'a mut Vec<OuterIndex>,
}

impl<'a> RandomOuterIndexRemover<'a> {
    pub fn find_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        // loop till the given outer index is found
        // - write the found value to cached_outer_index
        // - write other values to cache
        //
        // return true if found, false if the list concludes

        let side = self.side();

        let RandomOuterIndexRemover {
            inner: remover,
            cached_outer_indices,
        } = self;

        loop {
            if let Some(read_outer_index) = remover.active_outer_index_iterator.next(ctx) {
                if read_outer_index == outer_index {
                    remover.cached_outer_index = Some(read_outer_index);
                    return true;
                } else if side == Side::Bid && read_outer_index > outer_index
                    || side == Side::Ask && read_outer_index < outer_index
                {
                    cached_outer_indices.push(read_outer_index);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    pub fn write_index_list(&mut self, ctx: &mut ArbContext) {
        self.inner.commit();

        write_index_list(
            ctx,
            self.side(),
            &mut self.cached_outer_indices,
            self.inner.unread_outer_index_count(),
            self.inner.active_outer_index_iterator.list_slot,
        );
    }

    pub fn remove(&mut self) {
        self.inner.remove_cached_index();
    }

    pub fn side(&self) -> Side {
        self.inner.side()
    }
}
