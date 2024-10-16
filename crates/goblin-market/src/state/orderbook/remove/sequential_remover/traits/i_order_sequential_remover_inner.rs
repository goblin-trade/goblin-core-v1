use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        remove::IOuterIndexRemover,
        OuterIndex, Side,
    },
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

    /// Bitmap group must be written if active orders remain on the
    /// best price even after closing the bit, i.e. the best market price
    /// remains unchanged
    fn update_pending_write(&mut self, best_price_unchanged: bool) {
        *self.pending_write_mut() = best_price_unchanged;
    }

    // Getters

    /// The current outer index
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }
}
