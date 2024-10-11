use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, OuterIndex, Side,
    },
};

use super::{
    group_position_remover_v3::{
        GroupPositionRemoverV3, IGroupPositionRemover, RandomGroupPositionRemover,
        SequentialGroupPositionRemover,
    },
    random_outer_index_remover_v3::{IRandomOuterIndexRemover, RandomOuterIndexRemoverV3},
    sequential_order_remover_v3::ISequentialOrderRemoverV3,
    sequential_outer_index_remover_v3::{IOuterIndexRemover, ISequentialOuterIndexRemover},
};

pub struct RandomOrderRemoverV3<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: RandomOuterIndexRemoverV3<'a>,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemoverV3,

    /// Reference to best market price for current side from market state
    pub best_market_price: &'a mut Ticks,

    /// Whether the bitmap group is pending a write
    pub pending_write: bool,
}

impl<'a> RandomOrderRemoverV3<'a> {
    pub fn new(
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        RandomOrderRemoverV3 {
            outer_index_remover: RandomOuterIndexRemoverV3::new(side, outer_index_count),
            group_position_remover: GroupPositionRemoverV3::new(side),
            pending_write: false,
            best_market_price,
        }
    }
}

pub trait IRandomOrderRemoverV3<'a> {
    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl RandomGroupPositionRemover;

    /// Mutable reference to group position remover
    fn group_position_remover_mut(&mut self) -> &mut impl RandomGroupPositionRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IRandomOuterIndexRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IRandomOuterIndexRemover<'a>;

    /// Reference to best market price for current side from market state
    fn best_market_price_mut(&mut self) -> &mut Ticks;

    /// Whether the bitmap group is pending a write
    fn pending_write_mut(&mut self) -> &mut bool;

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

        if *self.pending_write_mut() {
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
            .paginate_and_check_if_active(GroupPosition::from(&order_id))
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
            if price == *self.best_market_price_mut()
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
                self.group_position_remover_mut().deactivate(group_position);

                let group_is_active = self.group_position_remover_mut().is_group_active();
                self.set_pending_write(group_is_active);
                if !group_is_active {
                    self.outer_index_remover_mut().remove();
                }
            }
        }
    }

    fn sequential_order_remover(&mut self) -> &mut impl ISequentialOrderRemoverV3<'a>;

    // Setters
    fn set_pending_write(&mut self, non_outermost_group_is_active: bool) {
        *self.pending_write_mut() = non_outermost_group_is_active;
    }

    // Getters
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }

    fn group_position(&self) -> Option<GroupPosition> {
        self.group_position_remover().group_position()
    }

    fn order_id(&self) -> Option<OrderId> {
        let outer_index = self.outer_index()?;
        let group_position = self.group_position()?;

        Some(OrderId::from_group_position(group_position, outer_index))
    }
}

impl<'a> IRandomOrderRemoverV3<'a> for RandomOrderRemoverV3<'a> {
    fn group_position_remover(&self) -> &impl RandomGroupPositionRemover {
        &self.group_position_remover
    }

    fn group_position_remover_mut(&mut self) -> &mut impl RandomGroupPositionRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover(&self) -> &impl IRandomOuterIndexRemover<'a> {
        &self.outer_index_remover
    }

    fn outer_index_remover_mut(&mut self) -> &mut impl IRandomOuterIndexRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price_mut(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }

    fn sequential_order_remover(&mut self) -> &mut impl ISequentialOrderRemoverV3<'a> {
        self
    }
}

impl<'a> ISequentialOrderRemoverV3<'a> for RandomOrderRemoverV3<'a> {
    fn group_position_remover_mut(&mut self) -> &mut impl SequentialGroupPositionRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover(&self) -> &impl ISequentialOuterIndexRemover<'a> {
        &self.outer_index_remover
    }

    fn outer_index_remover_mut(&mut self) -> &mut impl ISequentialOuterIndexRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price_mut(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}
