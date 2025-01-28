use crate::state::{remove::IOuterIndexRemover, ArbContext};

/// Removes outer indices one by one from the index list beginning from the
/// end of the list. Outer indices closer to the centre are removed first.
///
/// Sequential removal simply involves decrementing outer index count from
/// the market state. There is no need to cache values or perform slot writes.
pub trait IOuterIndexSequentialRemover<'a>: IOuterIndexRemover<'a> {
    /// Read the next outer index from index list and set it as current
    fn load_next(&mut self, ctx: &ArbContext) {
        *self.current_outer_index_mut() = self.active_outer_index_iterator_mut().next(ctx);
    }

    /// Concludes removals by adding the cached value back to the list
    ///
    /// This simply involves incrementing the count if a value is cached
    fn commit(&mut self) {
        *self
            .active_outer_index_iterator_mut()
            .inner
            .unread_outer_indices += u16::from(self.current_outer_index_mut().is_some());
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{
        remove::OuterIndexSequentialRemover, ContextActions, ListKey, ListSlot, Side,
    };

    use super::*;

    #[test]
    fn test_read_asks() {
        let ctx = &mut ArbContext::new();
        let side = Side::Ask;
        let mut outer_index_count = 17;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_item_1 = ListSlot {
            inner: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexSequentialRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.current_outer_index, None);

        for i in (0..=16).rev() {
            remover.load_next(ctx);
            assert_eq!(remover.current_outer_index.unwrap().as_u16(), i);
        }
        remover.load_next(ctx);
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(
            remover.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // No effect on commit since all values were cleared
        remover.commit();
        assert_eq!(outer_index_count, 0);
    }

    #[test]
    fn test_read_bids() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut outer_index_count = 17;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2],
        };
        let list_item_1 = ListSlot {
            inner: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexSequentialRemover::new(side, &mut outer_index_count);

        for i in 1..=17 {
            remover.load_next(ctx);
            assert_eq!(remover.current_outer_index.unwrap().as_u16(), i);
        }
        remover.load_next(ctx);
        assert_eq!(remover.current_outer_index, None);
        assert_eq!(
            remover.active_outer_index_iterator.unread_outer_indices(),
            0
        );

        // No effect on commit since all values were cleared
        remover.commit();
        assert_eq!(outer_index_count, 0);
    }

    #[test]
    fn test_read_commit_and_read_again() {
        let ctx = &mut ArbContext::new();
        let side = Side::Ask;
        let mut outer_index_count = 17;

        let list_key_0 = ListKey { index: 0, side };
        let list_key_1 = ListKey { index: 1, side };

        let list_item_0 = ListSlot {
            inner: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        };
        let list_item_1 = ListSlot {
            inner: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        list_item_0.write_to_slot(ctx, &list_key_0);
        list_item_1.write_to_slot(ctx, &list_key_1);

        let mut remover = OuterIndexSequentialRemover::new(side, &mut outer_index_count);
        assert_eq!(remover.current_outer_index, None);

        // Remove two values and commit
        remover.load_next(ctx);
        assert_eq!(
            remover.active_outer_index_iterator.unread_outer_indices(),
            16
        );

        remover.load_next(ctx);
        assert_eq!(
            remover.active_outer_index_iterator.unread_outer_indices(),
            15
        );

        remover.commit();
        assert_eq!(
            remover.active_outer_index_iterator.unread_outer_indices(),
            16
        );

        // New remover for remaining values
        let mut outer_index_count_new = remover.active_outer_index_iterator.unread_outer_indices();
        let mut remover_new = OuterIndexSequentialRemover::new(side, &mut outer_index_count_new);
        for i in (0..=15).rev() {
            remover_new.load_next(ctx);
            assert_eq!(remover_new.current_outer_index.unwrap().as_u16(), i);
        }

        remover_new.load_next(ctx);
        assert_eq!(remover_new.current_outer_index, None);
        assert_eq!(outer_index_count_new, 0);
    }
}
