use crate::{
    quantities::Ticks,
    state::{
        order::order_id::OrderId, write_index_list::write_index_list, ArbContext, OuterIndex, Side,
    },
};

use super::{
    group_position_remover_v2::GroupPositionRemoverV2,
    sequential_order_remover::SequentialOrderRemover,
    sequential_order_remover_v2::SequentialOrderRemoverV2,
    sequential_outer_index_remover::SequentialOuterIndexRemover,
};

use alloc::vec::Vec;

pub struct RandomOrderRemoverV2<'a> {
    inner: SequentialOrderRemoverV2<'a>,

    cached_outer_indices: Vec<OuterIndex>,
}

impl<'a> RandomOrderRemoverV2<'a> {
    pub fn new(
        side: Side,
        best_market_price: &'a mut Ticks,
        outer_index_count: &'a mut u16,
    ) -> Self {
        RandomOrderRemoverV2 {
            inner: SequentialOrderRemoverV2::new(side, best_market_price, outer_index_count),
            cached_outer_indices: Vec::new(),
        }
    }

    pub fn random_outer_index_remover(&'a mut self) -> RandomOuterIndexRemover {
        RandomOuterIndexRemover {
            inner: &mut self.inner.outer_index_remover,
            cached_outer_indices: &mut self.cached_outer_indices,
        }
    }
}

pub struct RandomOuterIndexRemover<'a> {
    inner: &'a mut SequentialOuterIndexRemover<'a>,
    cached_outer_indices: &'a mut Vec<OuterIndex>,
}

impl<'a> RandomOuterIndexRemover<'a> {
    pub fn find_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        // loop till the given outer index is found
        // - write the found value to cached_outer_index
        // - write other values to cache
        //
        // return true if found, false if the list concludes

        let side = self.side();

        let RandomOuterIndexRemover {
            inner: remover,
            cached_outer_indices,
        } = self;

        loop {
            if let Some(read_outer_index) = remover.active_outer_index_iterator.next(ctx) {
                if read_outer_index == outer_index {
                    remover.cached_outer_index = Some(read_outer_index);
                    return true;
                } else if side == Side::Bid && read_outer_index > outer_index
                    || side == Side::Ask && read_outer_index < outer_index
                {
                    cached_outer_indices.push(read_outer_index);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        // false
    }

    pub fn write_index_list(&mut self, ctx: &mut ArbContext) {
        self.inner.commit();

        write_index_list(
            ctx,
            self.side(),
            &mut self.cached_outer_indices,
            self.inner.unread_outer_index_count(),
            self.inner.active_outer_index_iterator.list_slot,
        );
    }

    pub fn remove(&mut self) {
        self.inner.remove_cached_index();
    }

    pub fn side(&self) -> Side {
        self.inner.side()
    }
}
