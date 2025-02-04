use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    remove::NextOuterIndexLoader, ArbContext, OuterIndex, Side,
};

/// Helper to sequentially read and remove outer indices from the index list
/// in slot storage
pub struct OuterIndexSequentialRemover<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,
}

impl<'a> OuterIndexSequentialRemover<'a> {
    /// Constructs a new SequentialOuterIndexRemover
    ///
    /// # Arguments
    ///
    /// * `side`
    /// * `outer_index_count` - Reference to outer index count for the given
    /// side in MarketState
    pub fn new(side: Side, outer_index_count: &'a mut u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIteratorV2::new(side, outer_index_count),
            current_outer_index: None,
        }
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    pub fn commit_sequential(&mut self) {
        *self.active_outer_index_iterator.inner.unread_outer_indices +=
            u16::from(self.current_outer_index.is_some());
    }
}

impl<'a> NextOuterIndexLoader for OuterIndexSequentialRemover<'a> {
    fn load_next(&mut self, ctx: &ArbContext) {
        self.current_outer_index = self.active_outer_index_iterator.next(ctx);
    }
}
