use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    ArbContext, OuterIndex, Side,
};

/// Helper to sequentially read and remove outer indices from the index list
/// in slot storage
pub struct SequentialOuterIndexRemoverV3<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,
}

pub trait ISequentialOuterIndexRemover<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a>;

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex>;

    /// Read the next outer index from index list and set it as current
    fn next(&mut self, ctx: &mut ArbContext) {
        *self.current_outer_index() = self.active_outer_index_iterator().next(ctx);
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    fn commit(&mut self) {
        *self.active_outer_index_iterator().inner.outer_index_count +=
            u16::from(self.current_outer_index().is_some());
    }
}

impl<'a> ISequentialOuterIndexRemover<'a> for SequentialOuterIndexRemoverV3<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}

impl<'a> SequentialOuterIndexRemoverV3<'a> {
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
}
