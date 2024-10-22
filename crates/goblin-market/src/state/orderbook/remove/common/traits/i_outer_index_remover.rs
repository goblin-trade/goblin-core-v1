use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    ArbContext, OuterIndex,
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

    /// Get the outermost outer index from the list. First take from
    /// cached outer index if the value is Some, else read from the index list iterator.
    fn next_outer_index(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
        if let Some(cached_outer_index) = self.current_outer_index_mut().take() {
            Some(cached_outer_index)
        } else {
            self.active_outer_index_iterator_mut().next(ctx)
        }
    }
}
