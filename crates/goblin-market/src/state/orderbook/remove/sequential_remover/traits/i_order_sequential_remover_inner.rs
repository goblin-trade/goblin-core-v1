use crate::{
    quantities::Ticks,
    state::{remove::IOuterIndexRemover, OuterIndex},
};

use super::{IGroupPositionSequentialRemover, IOuterIndexSequentialRemover};

pub trait IOrderSequentialRemoverInner<'a> {
    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl IGroupPositionSequentialRemover;

    /// Mutable reference to group position remover, to lookup and remove outer indices
    fn group_position_remover_mut(&mut self) -> &mut impl IGroupPositionSequentialRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IOuterIndexSequentialRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexSequentialRemover<'a>;

    /// The market price for current side from market state
    fn best_market_price_inner(&self) -> Ticks;

    /// Reference to best market price for current side from market state
    fn best_market_price_inner_mut(&mut self) -> &mut Ticks;

    /// Whether the bitmap group is pending a write
    fn pending_write(&self) -> bool;

    /// Mutable reference to pending write
    fn pending_write_mut(&mut self) -> &mut bool;

    /// Upates pending write state for bitmap group. If pending write is true
    /// when reads have concluded then we must write the bitmap group to slot.
    ///
    /// # Arguments
    ///
    /// * `is_first_read` - Nothing is removed on the first read since there is no
    /// previous value.
    /// * `best_price_unchanged` - If best market price did not update after closing
    /// the current bit, we must write the group to slot.
    fn update_pending_write(&mut self, is_first_read: bool, best_price_unchanged: bool) {
        *self.pending_write_mut() = !is_first_read && best_price_unchanged;
    }

    // Getters

    /// The current outer index
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }
}
