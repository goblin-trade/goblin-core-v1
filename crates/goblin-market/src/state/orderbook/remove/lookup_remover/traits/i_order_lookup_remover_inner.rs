use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        remove::{IGroupPositionRemover, IOrderSequentialRemover, IOuterIndexRemover},
        OuterIndex, Side,
    },
};

use super::{IGroupPositionLookupRemover, IOuterIndexLookupRemover};

pub trait IOrderLookupRemoverInner<'a> {
    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &impl IGroupPositionLookupRemover;

    /// Mutable reference to group position remover
    fn group_position_remover_mut(&mut self) -> &mut impl IGroupPositionLookupRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &impl IOuterIndexLookupRemover<'a>;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut impl IOuterIndexLookupRemover<'a>;

    /// Best market price for current side from market state
    fn best_market_price_inner(&self) -> Ticks;

    /// Reference to best market price for current side from market state
    fn best_market_price_inner_mut(&mut self) -> &mut Ticks;

    /// Whether the outer index changed and the bitmap group is pending a read
    fn pending_read(&self) -> bool;

    /// Mutable reference to pending read
    fn pending_read_mut(&mut self) -> &mut bool;

    /// Whether the bitmap group is pending a write
    fn pending_write(&self) -> bool;

    /// Mutable reference to pending write
    fn pending_write_mut(&mut self) -> &mut bool;

    fn sequential_order_remover(&mut self) -> &mut impl IOrderSequentialRemover<'a>;

    // Getters

    /// The current outer index. If the bitmap group becomes empty on removal
    /// then outer index is removed and this function returns None.
    ///
    /// Incoming order ids should be sorted by outer index, moving away from
    /// the centre. If the cached outer index is None due to its group closing,
    /// the next outer index is read from the bitmap list for comparison.
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
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
        self.group_position_remover().current_position()
    }

    fn side(&self) -> Side {
        self.group_position_remover().side()
    }

    fn order_id_to_remove(&self) -> Option<OrderId> {
        let outer_index = self.outer_index()?;
        let group_position = self.group_position()?;

        Some(OrderId::from_group_position(group_position, outer_index))
    }
}
