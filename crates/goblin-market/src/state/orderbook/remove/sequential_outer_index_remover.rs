use crate::state::{
    iterator::active_position::active_outer_index_iterator::ActiveOuterIndexIterator,
    write_index_list::write_index_list, ArbContext, OuterIndex, Side,
};
use alloc::vec::Vec;

pub struct SequentialOuterIndexRemover<'a> {
    /// Iterator to read active outer indices from index list
    pub active_outer_index_iterator: ActiveOuterIndexIterator,

    /// The currently read outer index
    pub cached_outer_index: Option<OuterIndex>,

    /// Total outer index count in market state
    pub outer_index_count: &'a mut u16,
}

impl<'a> SequentialOuterIndexRemover<'a> {
    pub fn new(side: Side, outer_index_count: &'a mut u16) -> Self {
        Self {
            active_outer_index_iterator: ActiveOuterIndexIterator::new(side, *outer_index_count),
            cached_outer_index: None,
            outer_index_count,
        }
    }

    pub fn next(&mut self, ctx: &mut ArbContext) -> Option<OuterIndex> {
        self.slide(ctx);
        self.cached_outer_index
    }

    pub fn slide(&mut self, ctx: &mut ArbContext) {
        // TODO will sliding on empty list lead to decrease in index_list_length()
        // because cached_outer_index was cleared?
        self.cached_outer_index = self.active_outer_index_iterator.next(ctx);
    }

    pub fn write_index_list(&mut self) {
        *self.outer_index_count = self.index_list_length();
    }

    // Getters

    pub fn side(&self) -> Side {
        self.active_outer_index_iterator.side
    }

    /// The total length of index list after accounting for removals
    pub fn index_list_length(&self) -> u16 {
        self.active_outer_index_iterator.outer_index_count()
            + u16::from(self.cached_outer_index.is_some())
    }

    /// Whether the currently cached outer index belongs to the outermost bitmap group
    pub fn on_outermost_index(&self) -> bool {
        self.index_list_length() == *self.outer_index_count && self.cached_outer_index.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{ContextActions, ListKey, ListSlot};

    use super::*;

    #[test]
    fn sequentially_remove_bids() {
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
        assert_eq!(remover.on_outermost_index(), true);
        assert_eq!(remover.index_list_length(), 4);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 3);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 2);

        let index_3 = remover.next(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(1));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 1);

        let index_4 = remover.next(ctx);
        assert!(index_4.is_none());
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 0);

        remover.write_index_list();
        assert_eq!(*remover.outer_index_count, 0);
    }

    #[test]
    fn sequentially_remove_asks() {
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
        assert_eq!(remover.on_outermost_index(), true);
        assert_eq!(remover.index_list_length(), 4);

        let index_1 = remover.next(ctx);
        assert_eq!(index_1.unwrap(), OuterIndex::new(2));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 3);

        let index_2 = remover.next(ctx);
        assert_eq!(index_2.unwrap(), OuterIndex::new(3));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 2);

        let index_3 = remover.next(ctx);
        assert_eq!(index_3.unwrap(), OuterIndex::new(4));
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 1);

        let index_4 = remover.next(ctx);
        assert!(index_4.is_none());
        assert_eq!(remover.on_outermost_index(), false);
        assert_eq!(remover.index_list_length(), 0);

        remover.write_index_list();
        assert_eq!(*remover.outer_index_count, 0);
    }
}
