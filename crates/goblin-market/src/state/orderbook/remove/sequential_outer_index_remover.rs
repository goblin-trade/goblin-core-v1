use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    ArbContext, OuterIndex, Side,
};

/// Helper to sequentially read and remove outer indices from the index list
/// in slot storage
pub struct SequentialOuterIndexRemover<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub current_outer_index: Option<OuterIndex>,
}

pub trait ISequentialOuterIndexRemover<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a>;

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex>;

    fn load_outer_index(&'a mut self, ctx: &mut ArbContext) {
        *self.current_outer_index() = self.active_outer_index_iterator().next(ctx);
    }
}

impl<'a> ISequentialOuterIndexRemover<'a> for SequentialOuterIndexRemover<'a> {
    fn active_outer_index_iterator(&mut self) -> &mut ActiveOuterIndexIteratorV2<'a> {
        &mut self.active_outer_index_iterator
    }

    fn current_outer_index(&mut self) -> &mut Option<OuterIndex> {
        &mut self.current_outer_index
    }
}

impl<'a> SequentialOuterIndexRemover<'a> {
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

    // Common functions

    /// Remove the currently loaded index
    /// TODO rewrite SequentialOrderRemoverV2 to avoid this function
    pub fn remove_loaded_index(&mut self) {
        self.current_outer_index = None;
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    pub fn commit(&mut self) {
        *self.unread_outer_index_count_mut() += u16::from(self.current_outer_index.is_some());
    }

    // Sequential functions

    /// Returns the current cached outer index if present, else tries to load and
    /// return the next value
    pub fn get_or_load_outer_index(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
        if self.current_outer_index.is_some() {
            return self.current_outer_index;
        }

        self.load_and_return_outer_index(ctx)
    }

    /// Load and return the next outer index from index list
    ///
    /// Calling this function after the last value was loaded will clear
    /// current_outer_index and return None
    fn load_and_return_outer_index(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
        self.current_outer_index = self.active_outer_index_iterator.next(ctx);
        self.current_outer_index
    }

    /// Mutable reference to outer index count from market state
    /// This value is equal to the number of elements yet to be read from the index list
    pub fn unread_outer_index_count_mut(&mut self) -> &mut u16 {
        self.active_outer_index_iterator.inner.outer_index_count
    }

    // Getters

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{ContextActions, ListKey, ListSlot};

    use super::*;

    fn write_outer_indices(ctx: &mut ArbContext, side: Side, indices: Vec<u16>) {
        let slot_count = indices.len() / 16;

        for i in 0..=slot_count {
            let list_key = ListKey {
                index: i as u16,
                side,
            };
            let mut list_slot = ListSlot::default();
            for j in 0..16 {
                let outer_index = indices.get(i * 16 + j);

                if let Some(outer_index) = outer_index {
                    list_slot.set(j, OuterIndex::new(*outer_index));
                } else {
                    break;
                }
            }
            list_slot.write_to_slot(ctx, &list_key);
        }
    }

    #[test]
    fn sequentially_remove_all_bids() {
        let ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let indices = vec![1, 2, 3, 4];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(*remover.unread_outer_index_count_mut(), 4);

        let index_0 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(4));
        assert_eq!(*remover.unread_outer_index_count_mut(), 3);

        let index_1 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(3));
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        let index_2 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(2));
        assert_eq!(*remover.unread_outer_index_count_mut(), 1);

        let index_3 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(1));
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);

        let index_4 = remover.load_and_return_outer_index(ctx);
        assert!(index_4.is_none());
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);

        remover.commit();
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);
    }

    #[test]
    fn sequentially_remove_some_bids() {
        let ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let indices = vec![1, 2, 3, 4];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(*remover.unread_outer_index_count_mut(), 4);

        let index_0 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(4));
        assert_eq!(*remover.unread_outer_index_count_mut(), 3);

        let index_1 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(3));
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        let index_2 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(2));
        assert_eq!(*remover.unread_outer_index_count_mut(), 1);

        remover.commit();
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        // spec- commit will not remove the outer index
        assert_eq!(remover.current_outer_index.unwrap(), OuterIndex::new(2));
    }

    #[test]
    fn sequentially_remove_all_asks() {
        let ctx = &mut ArbContext::new();

        let side = Side::Ask;
        let indices = vec![4, 3, 2, 1];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(*remover.unread_outer_index_count_mut(), 4);

        let index_0 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(1));
        assert_eq!(*remover.unread_outer_index_count_mut(), 3);

        let index_1 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(2));
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        let index_2 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(3));
        assert_eq!(*remover.unread_outer_index_count_mut(), 1);

        let index_3 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(4));
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);

        let index_4 = remover.load_and_return_outer_index(ctx);
        assert!(index_4.is_none());
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);

        remover.commit();
        assert_eq!(*remover.unread_outer_index_count_mut(), 0);
    }

    #[test]
    fn sequentially_remove_some_asks() {
        let ctx = &mut ArbContext::new();

        let side = Side::Ask;
        let indices = vec![4, 3, 2, 1];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);

        let index_0 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(1));
        assert_eq!(*remover.unread_outer_index_count_mut(), 3);

        let index_1 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(2));
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        let index_2 = remover.load_and_return_outer_index(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(3));
        assert_eq!(*remover.unread_outer_index_count_mut(), 1);

        remover.commit();
        assert_eq!(*remover.unread_outer_index_count_mut(), 2);

        // spec- commit will not remove the outer index
        assert_eq!(remover.current_outer_index.unwrap(), OuterIndex::new(3));
    }
}
