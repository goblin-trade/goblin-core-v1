use crate::{
    quantities::Ticks,
    state::{
        remove::{
            GroupPositionRemover, IGroupPositionRemover, IGroupPositionSequentialRemover,
            IOrderSequentialRemover, IOrderSequentialRemoverInner, IOuterIndexSequentialRemover,
        },
        ArbContext, BestPriceAndIndexCount, Side,
    },
};

use super::OuterIndexSequentialRemover;

/// Manager to sequentially read and remove orders, moving away from centre
/// of the book
pub struct OrderSequentialRemover<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexSequentialRemover<'a>,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemover,

    /// Reference to best market price for current side from market state
    pub best_market_price_inner: &'a mut Ticks,

    /// Whether the bitmap group is pending a write
    pub pending_write: bool,
}

impl<'a> OrderSequentialRemover<'a> {
    pub fn new(
        ctx: &ArbContext,
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        let mut outer_index_remover = OuterIndexSequentialRemover::new(side, outer_index_count);
        outer_index_remover.load_next(ctx);

        let mut group_position_remover = GroupPositionRemover::new(side);
        group_position_remover.load_outermost_group(ctx, *best_market_price);

        OrderSequentialRemover {
            outer_index_remover,
            group_position_remover,
            pending_write: false,
            best_market_price_inner: best_market_price,
        }
    }

    pub fn new_v2(side: Side, best_price_and_index_count: &'a mut BestPriceAndIndexCount) -> Self {
        OrderSequentialRemover {
            outer_index_remover: OuterIndexSequentialRemover::new(
                side,
                &mut best_price_and_index_count.outer_index_count,
            ),
            group_position_remover: GroupPositionRemover::new(side),
            pending_write: false,
            best_market_price_inner: &mut best_price_and_index_count.best_price_inner,
        }
    }
}

impl<'a> IOrderSequentialRemover<'a> for OrderSequentialRemover<'a> {
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

    fn best_market_price_inner(&self) -> Ticks {
        *self.best_market_price_inner
    }

    fn best_market_price_inner_mut(&mut self) -> &mut Ticks {
        &mut self.best_market_price_inner
    }

    fn pending_write(&self) -> bool {
        self.pending_write
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}

// impl<'a> IOrderSequentialRemover<'a> for OrderSequentialRemover<'a> {}
