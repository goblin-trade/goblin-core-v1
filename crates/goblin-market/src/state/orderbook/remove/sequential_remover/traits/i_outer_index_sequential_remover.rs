use crate::state::{remove::IOuterIndexRemover, ArbContext};

pub trait IOuterIndexSequentialRemover<'a>: IOuterIndexRemover<'a> {
    /// Read the next outer index from index list and set it as current
    fn next(&mut self, ctx: &mut ArbContext) {
        *self.current_outer_index_mut() = self.active_outer_index_iterator_mut().next(ctx);
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    fn commit(&mut self) {
        *self
            .active_outer_index_iterator_mut()
            .inner
            .outer_index_count += u16::from(self.current_outer_index_mut().is_some());
    }
}
