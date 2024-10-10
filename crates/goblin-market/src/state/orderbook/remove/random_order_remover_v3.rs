use crate::{
    quantities::Ticks,
    state::{
        order::{group_position::GroupPosition, order_id::OrderId},
        ArbContext, OuterIndex, Side,
    },
};

use super::{
    group_position_remover_v2::{
        GroupPositionRemoverV2, RandomGroupPositionRemover, SequentialGroupPositionRemover,
    },
    random_outer_index_remover_v3::{IRandomOuterIndexRemover, RandomOuterIndexRemoverV3},
    sequential_order_remover_v3::ISequentialOrderRemoverV3,
    sequential_outer_index_remover_v3::ISequentialOuterIndexRemover,
};

use alloc::vec::Vec;

pub struct RandomOrderRemoverV3<'a> {
    /// To lookup and remove outer indices
    pub outer_index_remover: RandomOuterIndexRemoverV3<'a>,

    /// To lookup and deactivate bits in bitmap groups
    pub group_position_remover: GroupPositionRemoverV2,

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
            group_position_remover: GroupPositionRemoverV2::new(side),
            pending_write: false,
            best_market_price,
        }
    }
}

// pub trait IRandomOrderRemover<'a> {
//     fn outer_index_remover(&mut self) -> &mut impl IRandomOuterIndexRemover<'a>;

//     fn group_position_remover(&mut self) -> &mut GroupPositionRemoverV2;

//     fn best_market_price(&mut self) -> &mut Ticks;

//     fn pending_write(&mut self) -> &mut bool;
// }

impl<'a> ISequentialOrderRemoverV3<'a> for RandomOrderRemoverV3<'a> {
    fn group_position_remover(&mut self) -> &mut impl SequentialGroupPositionRemover {
        &mut self.group_position_remover
    }

    fn outer_index_remover(&mut self) -> &mut impl ISequentialOuterIndexRemover<'a> {
        &mut self.outer_index_remover
    }

    fn best_market_price(&mut self) -> &mut Ticks {
        &mut self.best_market_price
    }

    fn pending_write(&mut self) -> &mut bool {
        &mut self.pending_write
    }
}
