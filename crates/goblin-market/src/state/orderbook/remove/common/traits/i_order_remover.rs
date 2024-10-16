use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, OuterIndex,
    },
};

use super::{IGroupPositionRemover, IOuterIndexRemover};

// pub trait IGroupPositionRemoverInner {
//     fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex);
// }

// pub trait IOuterIndexRemoverInner {
//     fn current_outer_index(&self) -> Option<OuterIndex>;
//     fn commit(&mut self, ctx: &mut ArbContext);
// }

pub trait IOrderRemoverInner<
    'a,
    GPRemover: IGroupPositionRemover,
    OIRemover: IOuterIndexRemover<'a>,
>
{
    /// To lookup and remove outer indices
    fn group_position_remover(&self) -> &GPRemover;

    /// Mutable reference to group position remover
    fn group_position_remover_mut(&mut self) -> &mut GPRemover;

    /// To lookup and deactivate bits in bitmap groups
    fn outer_index_remover(&self) -> &OIRemover;

    /// Mutable reference to outer index remover
    fn outer_index_remover_mut(&mut self) -> &mut OIRemover;

    /// The market price for current side from market state
    fn best_market_price_inner(&self) -> Ticks;

    /// Reference to best market price for current side from market state
    fn best_market_price_inner_mut(&mut self) -> &mut Ticks;

    /// Whether the bitmap group is pending a write
    fn pending_write(&self) -> bool;

    /// Mutable reference to pending write
    fn pending_write_mut(&mut self) -> &mut bool;

    /// The current outer index
    fn outer_index(&self) -> Option<OuterIndex> {
        self.outer_index_remover().current_outer_index()
    }
}
