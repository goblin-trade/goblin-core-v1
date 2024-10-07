use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, InnerIndex, MarketPrices, OuterIndex, RestingOrderIndex, Side,
    },
};

use super::{
    group_position_remover_v2::GroupPositionRemoverV2,
    outer_index_remover::OuterIndexRemover,
    sequential_outer_index_remover::{ISequentialOuterIndexRemover, SequentialOuterIndexRemover},
};

pub struct SequentialOrderRemoverV2<'a, T>
where
    T: ISequentialOuterIndexRemover + 'a,
{
    /// A field that holds a type implementing the ISequentialOuterIndexRemover trait
    pub outer_index_remover: T,

    pub best_market_price: &'a mut Ticks,

    pub pending_write: bool,
}

impl<'a, T> SequentialOrderRemoverV2<'a, T>
where
    T: ISequentialOuterIndexRemover + 'a,
{
    // /// A method to access the `next` method from the `remover`
    // pub fn process_next(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
    //     self.outer_index_remover.next(ctx)
    // }
}
