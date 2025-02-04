use crate::{
    quantities::Ticks,
    state::{
        iterator::active_position::active_group_position_iterator::ActiveGroupPositionIterator,
        remove::{GroupPositionSequentialRemover, NextOrderIterator, NextOuterIndexLoader},
        ArbContext, OuterIndex, Side,
    },
};

use super::OuterIndexSequentialRemover;

/// Manager to sequentially read and remove orders, moving away from centre
/// of the book
pub struct OrderSequentialRemover<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: OuterIndexSequentialRemover<'a>,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: ActiveGroupPositionIterator,

    /// Reference to best market price for current side from market state
    pub best_market_price: &'a mut Ticks,

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

        let mut group_position_remover = ActiveGroupPositionIterator::new(side);
        group_position_remover.load_outermost_group(ctx, *best_market_price);

        OrderSequentialRemover {
            outer_index_remover,
            group_position_remover,
            pending_write: false,
            best_market_price,
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
    /// IOrderLookupRemover::commit() has a similar looking function but it passes
    /// ctx to outer_index_remover_mut.commit() while the sequential remover does not.
    ///
    /// # Arguments
    ///
    /// * `ctx`
    ///
    pub fn commit(&mut self, ctx: &mut ArbContext) {
        if let Some(outer_index) = self.outer_index_remover.current_outer_index {
            if self.pending_write {
                self.group_position_remover
                    .bitmap_group_mut()
                    .write_to_slot(ctx, &outer_index);
            }

            // difference- ctx not passed to commit()
            self.outer_index_remover.commit_sequential();
        }
    }
}

impl<'a> NextOrderIterator<'a> for OrderSequentialRemover<'a> {
    fn group_position_sequential_remover(&mut self) -> &mut ActiveGroupPositionIterator {
        &mut self.group_position_remover
    }

    fn current_outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover.current_outer_index
    }

    fn next_outer_index_loader(&mut self) -> &mut impl NextOuterIndexLoader {
        &mut self.outer_index_remover
    }

    fn best_market_price(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write_mut(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}
