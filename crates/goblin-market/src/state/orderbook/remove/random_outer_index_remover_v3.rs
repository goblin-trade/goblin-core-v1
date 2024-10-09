use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    ArbContext, OuterIndex, Side,
};

use alloc::vec::Vec;

use super::sequential_outer_index_remover::{
    ISequentialOuterIndexRemover, SequentialOuterIndexRemover,
};

pub struct RandomOuterIndexRemoverV3<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,

    pub cached_outer_indices: Vec<OuterIndex>,
}

// impl<'a> ISequentialOuterIndexRemover<'a> for RandomOuterIndexRemoverV3<'a> {
//     fn active_outer_index_iterator(&'a mut self) -> &mut ActiveOuterIndexIteratorV2 {
//         &mut self.active_outer_index_iterator
//     }

//     fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
//         &mut self.current_outer_index
//     }
// }
