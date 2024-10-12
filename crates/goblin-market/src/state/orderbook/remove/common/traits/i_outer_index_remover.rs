use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    OuterIndex,
};

pub trait IOuterIndexRemover<'a> {
    /// Readonly reference to ActiveOuterIndexIteratorV2
    fn active_outer_index_iterator(&self) -> &ActiveOuterIndexIteratorV2<'a>;

    /// Iterator to read active outer indices from index list
    fn active_outer_index_iterator_mut(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a>;

    /// The currently read outer index
    fn current_outer_index(&self) -> Option<OuterIndex>;

    /// Mutable reference to the currently read outer index
    fn current_outer_index_mut(&mut self) -> &mut Option<OuterIndex>;
}
