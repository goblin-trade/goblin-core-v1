use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    remove::{IOuterIndexLookupRemover, IOuterIndexRemover, IOuterIndexSequentialRemover},
    OuterIndex, Side,
};
use alloc::vec::Vec;

/// Lookup and delete remove arbitrary outer indices. The outer
/// indices must be sorted in a direction moving away from centre of the book
pub struct OuterIndexLookupRemover<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,

    /// Outer indices that need to be written back to the list
    pub cached_outer_indices: Vec<OuterIndex>,
}

impl<'a> OuterIndexLookupRemover<'a> {
    /// Constructs a new RandomOuterIndexRemover
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
            cached_outer_indices: Vec::new(),
        }
    }
}

impl<'a> IOuterIndexRemover<'a> for OuterIndexLookupRemover<'a> {
    fn active_outer_index_iterator(&self) -> &ActiveOuterIndexIteratorV2<'a> {
        &self.active_outer_index_iterator
    }

    fn active_outer_index_iterator_mut(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&self) -> Option<OuterIndex> {
        self.current_outer_index
    }

    fn current_outer_index_mut(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}

impl<'a> IOuterIndexSequentialRemover<'a> for OuterIndexLookupRemover<'a> {}

impl<'a> IOuterIndexLookupRemover<'a> for OuterIndexLookupRemover<'a> {
    fn cached_outer_indices(&self) -> &Vec<OuterIndex> {
        &self.cached_outer_indices
    }

    fn cached_outer_indices_mut(&mut self) -> &mut Vec<OuterIndex> {
        &mut self.cached_outer_indices
    }
}
