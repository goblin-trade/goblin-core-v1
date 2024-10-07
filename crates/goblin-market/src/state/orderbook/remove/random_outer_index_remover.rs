use super::sequential_outer_index_remover::SequentialOuterIndexRemover;
use crate::state::{write_index_list::write_index_list, ArbContext, OuterIndex, Side};
use alloc::vec::Vec;

/// Two step remover to lookup and remove a list outer indices
/// arranged in a direction that moves away from the centre of the book
pub struct RandomOuterIndexRemover<'a> {
    inner: SequentialOuterIndexRemover<'a>,

    cache: Vec<OuterIndex>,
}

impl<'a> RandomOuterIndexRemover<'a> {
    pub fn new(side: Side, outer_index_count: &'a mut u16) -> Self {
        Self {
            inner: SequentialOuterIndexRemover::new(side, outer_index_count),
            cache: Vec::new(),
        }
    }

    pub fn find(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex) -> bool {
        // loop till the given outer index is found
        // - write the found value to cached_outer_index
        // - write other values to cache
        //
        // return true if found, false if the list concludes

        loop {
            if let Some(read_outer_index) = self.inner.active_outer_index_iterator.next(ctx) {
                if read_outer_index == outer_index {
                    self.inner.cached_outer_index = Some(read_outer_index);
                    return true;
                } else if self.side() == Side::Bid && read_outer_index > outer_index
                    || self.side() == Side::Ask && read_outer_index < outer_index
                {
                    self.cache.push(read_outer_index);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    pub fn remove(&mut self) {
        self.inner.cached_outer_index = None;
    }

    pub fn write_index_list(&mut self, ctx: &mut ArbContext) {
        self.inner.commit();

        write_index_list(
            ctx,
            self.inner.side(),
            &mut self.cache,
            self.inner.unread_outer_index_count(),
            self.inner.active_outer_index_iterator.list_slot,
        );
    }

    pub fn side(&self) -> Side {
        self.inner.side()
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
    fn test_find_absent_value() {
        let ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let indices = vec![1, 2, 3, 5, 6];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = RandomOuterIndexRemover::new(side, &mut outer_index_count);

        let found = remover.find(ctx, OuterIndex::new(4));
        assert_eq!(found, false);

        assert!(remover.inner.cached_outer_index.is_none());
        assert_eq!(remover.cache, vec![OuterIndex::new(6), OuterIndex::new(5)]);
    }

    #[test]
    fn test_find_and_remove_bids() {
        let ctx = &mut ArbContext::new();

        let side = Side::Bid;
        let indices = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let mut outer_index_count = indices.len() as u16;
        write_outer_indices(ctx, side, indices);

        let mut remover = RandomOuterIndexRemover::new(side, &mut outer_index_count);

        // 1. Find and outermost
        remover.find(ctx, OuterIndex::new(19));
        assert_eq!(
            remover.inner.cached_outer_index.unwrap(),
            OuterIndex::new(19)
        );
        assert!(remover.cache.is_empty());
        assert_eq!(remover.inner.unread_outer_index_count(), 18);

        remover.remove();
        assert!(remover.inner.cached_outer_index.is_none());
        assert_eq!(remover.inner.unread_outer_index_count(), 18);

        // 2. TODO find and remove from different slot
    }
}
