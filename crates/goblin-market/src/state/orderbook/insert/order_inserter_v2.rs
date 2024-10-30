use crate::{
    quantities::Ticks,
    state::{order::order_id::OrderId, ArbContext, OuterIndex, Side},
};

use super::{GroupPositionInserterV2, OuterIndexInserterV2};

/// Activate order ids in bulk
///
/// Outer indices of successive order ids must move away from the centre.
///
/// # Unknowns
///
/// * Garbage bits
/// * Update best market price
/// * If the last written outer index goes into the cache list, how to know the last
/// outer index?
pub struct OrderInserterV2<'a> {
    group_position_inserter: GroupPositionInserterV2,
    outer_index_inserter: OuterIndexInserterV2<'a>,
    best_market_price: &'a mut Ticks,
    // TODO need opposite price to clear garbage bits?
}

impl<'a> OrderInserterV2<'a> {
    pub fn new(
        side: Side,
        outer_index_count: &'a mut u16,
        best_market_price: &'a mut Ticks,
    ) -> Self {
        OrderInserterV2 {
            group_position_inserter: GroupPositionInserterV2::new(),
            outer_index_inserter: OuterIndexInserterV2::new(side, outer_index_count),
            best_market_price,
        }
    }

    // Design change
    // Previously we lookup for vacant order ids, then call insert. Instead
    // we should just pass the required price to insert() and let the inserter figure
    // out the best available position.
    //

    /// Try to insert an order at a given tick
    pub fn insert(&mut self, ctx: &mut ArbContext, price_in_ticks: Ticks) -> Option<OrderId> {
        let outer_index = price_in_ticks.outer_index();

        // Exit if out of order
        if self
            .last_added_outer_index()
            .is_some_and(|last_outer_index| {
                outer_index.is_closer_to_center(self.side(), last_outer_index)
            })
        {
            return None;
        }

        let outer_index_inserted = self.outer_index_inserter.insert_if_absent(ctx, outer_index);

        if outer_index_inserted {
            // Entire group is free. Insertion should be successful
        } else {
            // Check whether a slot is free on `price_in_ticks`
            // What about adjustment if no slots are free?
            // If adjustment fails how to revert the outer index insertion?
        }

        return None;
    }

    // Getters
    fn side(&self) -> Side {
        self.outer_index_inserter.side()
    }

    fn last_added_outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_inserter.last_added_outer_index()
    }
}
