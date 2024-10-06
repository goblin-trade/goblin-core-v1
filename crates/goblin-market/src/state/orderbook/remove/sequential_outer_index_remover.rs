use crate::state::{
    iterator::active_position::active_outer_index_iterator_v2::ActiveOuterIndexIteratorV2,
    ArbContext, OuterIndex, Side,
};

pub struct SequentialOuterIndexRemover<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIteratorV2<'a>,

    /// The currently read outer index
    pub cached_outer_index: Option<OuterIndex>,
}

impl<'a> SequentialOuterIndexRemover<'a> {
    pub fn new(side: Side, outer_index_count: &'a mut u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIteratorV2::new(side, outer_index_count),
            cached_outer_index: None,
        }
    }

    /// Cache and return the next value in the index list
    pub fn next(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
        self.slide(ctx);
        self.cached_outer_index
    }

    /// Read and cache the next outer index
    ///
    /// Once the last element is read and loaded in to cached_outer_index,
    /// calling slide() again will clear the value
    pub fn slide(&mut self, ctx: &mut ArbContext) {
        self.cached_outer_index = self.active_outer_index_iterator.next(ctx);
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    pub fn write_index_list(&mut self) {
        *self.unread_outer_index_count_mut() += u16::from(self.cached_outer_index.is_some())
    }

    /// Mutable reference to outer index count from market state
    /// This value is equal to the number of elements yet to be read from the index list
    pub fn unread_outer_index_count_mut(&mut self) -> &mut u16 {
        self.active_outer_index_iterator.inner.outer_index_count
    }

    pub fn unread_outer_index_count(&self) -> u16 {
        *self.active_outer_index_iterator.inner.outer_index_count
    }

    // Getters

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }

    /// The total length of index list after accounting for removals
    pub fn index_list_length(&self) -> u16 {
        self.unread_outer_index_count() + u16::from(self.cached_outer_index.is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{ContextActions, ListKey, ListSlot};

    use super::*;

    #[test]
    fn sequentially_remove_all_bids() {
        let mut ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let mut outer_index_count = 4;

        // Setup the initial slot storage with one item
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(1));
        list_slot.set(1, OuterIndex::new(2));
        list_slot.set(2, OuterIndex::new(3));
        list_slot.set(3, OuterIndex::new(4));
        list_slot.write_to_slot(&mut ctx, &list_key);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.index_list_length(), 4);

        let index_0 = remover.next(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(4));
        assert_eq!(remover.index_list_length(), 4);
        assert_eq!(remover.unread_outer_index_count(), 3);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.index_list_length(), 3);
        assert_eq!(remover.unread_outer_index_count(), 2);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.index_list_length(), 2);
        assert_eq!(remover.unread_outer_index_count(), 1);

        let index_3 = remover.next(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(1));
        assert_eq!(remover.index_list_length(), 1);
        assert_eq!(remover.unread_outer_index_count(), 0);

        let index_4 = remover.next(ctx);
        assert!(index_4.is_none());
        assert_eq!(remover.index_list_length(), 0);
        assert_eq!(remover.unread_outer_index_count(), 0);

        remover.write_index_list();
        assert_eq!(remover.unread_outer_index_count(), 0);
    }

    #[test]
    fn sequentially_remove_some_bids() {
        let mut ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let mut outer_index_count = 4;

        // Setup the initial slot storage with one item
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(1));
        list_slot.set(1, OuterIndex::new(2));
        list_slot.set(2, OuterIndex::new(3));
        list_slot.set(3, OuterIndex::new(4));
        list_slot.write_to_slot(&mut ctx, &list_key);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.index_list_length(), 4);

        let index_0 = remover.next(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(4));
        assert_eq!(remover.index_list_length(), 4);
        assert_eq!(remover.unread_outer_index_count(), 3);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.index_list_length(), 3);
        assert_eq!(remover.unread_outer_index_count(), 2);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.index_list_length(), 2);
        assert_eq!(remover.unread_outer_index_count(), 1);

        remover.write_index_list();
        assert_eq!(remover.unread_outer_index_count(), 2);
    }

    #[test]
    fn sequentially_remove_all_asks() {
        let mut ctx = &mut ArbContext::new();

        let side = Side::Ask;
        let mut outer_index_count = 4;

        // Setup the initial slot storage with one item
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(4));
        list_slot.set(1, OuterIndex::new(3));
        list_slot.set(2, OuterIndex::new(2));
        list_slot.set(3, OuterIndex::new(1));
        list_slot.write_to_slot(&mut ctx, &list_key);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.index_list_length(), 4);

        let index_0 = remover.next(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(1));
        assert_eq!(remover.index_list_length(), 4);
        assert_eq!(remover.unread_outer_index_count(), 3);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.index_list_length(), 3);
        assert_eq!(remover.unread_outer_index_count(), 2);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.index_list_length(), 2);
        assert_eq!(remover.unread_outer_index_count(), 1);

        let index_3 = remover.next(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(4));
        assert_eq!(remover.index_list_length(), 1);
        assert_eq!(remover.unread_outer_index_count(), 0);

        let index_4 = remover.next(ctx);
        assert!(index_4.is_none());
        assert_eq!(remover.index_list_length(), 0);
        assert_eq!(remover.unread_outer_index_count(), 0);

        remover.write_index_list();
        assert_eq!(remover.unread_outer_index_count(), 0);
    }

    #[test]
    fn sequentially_remove_some_asks() {
        let mut ctx = &mut ArbContext::new();

        let side = Side::Ask;
        let mut outer_index_count = 4;

        // Setup the initial slot storage with one item
        let list_key = ListKey { index: 0, side };
        let mut list_slot = ListSlot::default();
        list_slot.set(0, OuterIndex::new(4));
        list_slot.set(1, OuterIndex::new(3));
        list_slot.set(2, OuterIndex::new(2));
        list_slot.set(3, OuterIndex::new(1));
        list_slot.write_to_slot(&mut ctx, &list_key);

        let mut remover = SequentialOuterIndexRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.index_list_length(), 4);

        let index_0 = remover.next(ctx);
        assert_eq!(index_0.unwrap(), OuterIndex::new(1));
        assert_eq!(remover.index_list_length(), 4);
        assert_eq!(remover.unread_outer_index_count(), 3);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.index_list_length(), 3);
        assert_eq!(remover.unread_outer_index_count(), 2);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.index_list_length(), 2);
        assert_eq!(remover.unread_outer_index_count(), 1);

        remover.write_index_list();
        assert_eq!(remover.unread_outer_index_count(), 2);
    }
}
