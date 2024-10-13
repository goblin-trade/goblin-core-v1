use crate::{
    quantities::Ticks,
    state::{
        remove::{
            GroupPositionRemover, IGroupPositionLookupRemover, IGroupPositionSequentialRemover,
            IOrderLookupRemover, IOrderSequentialRemover, IOuterIndexLookupRemover,
            IOuterIndexSequentialRemover,
        },
        Side,
    },
};

use super::OuterIndexLookupRemover;

pub struct OrderLookupRemover<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexLookupRemover<'a>,

    /// To lookup and deactivate bits in bitmap groups
    /// TODO use IGroupPositionLookupRemover?
    pub group_position_remover: GroupPositionRemover,

    /// Reference to best market price for current side from market state
    pub best_market_price: &'a mut Ticks,

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
            pending_write: false,
            best_market_price,
        }
    }
}

impl<'a> IOrderSequentialRemover<'a> for OrderLookupRemover<'a> {
    fn group_position_remover(&self) -> &impl IGroupPositionSequentialRemover {
        &self.group_position_remover
    }

    fn group_position_remover_mut(&mut self) -> &mut impl IGroupPositionSequentialRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover(&self) -> &impl IOuterIndexSequentialRemover<'a> {
        &self.outer_index_remover
    }

    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexSequentialRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price_mut(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}

impl<'a> IOrderLookupRemover<'a> for OrderLookupRemover<'a> {
    fn group_position_remover(&self) -> &impl IGroupPositionLookupRemover {
        &self.group_position_remover
    }

    fn group_position_remover_mut(&mut self) -> &mut impl IGroupPositionLookupRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover(&self) -> &impl IOuterIndexLookupRemover<'a> {
        &self.outer_index_remover
    }

    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexLookupRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price_mut(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write(&self) -> bool {
        self.pending_write
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }

    fn sequential_order_remover(&mut self) -> &mut impl IOrderSequentialRemover<'a> {
        self
    }
}
