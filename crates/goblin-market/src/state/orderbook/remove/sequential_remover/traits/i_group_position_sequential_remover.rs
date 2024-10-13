use crate::state::{order::group_position::GroupPosition, remove::IGroupPositionRemover};

pub trait IGroupPositionSequentialRemover: IGroupPositionRemover {
    /// Deactivate current position and get next
    fn next(&mut self) -> Option<GroupPosition>;

    /// Whether the remover is still uninitialized
    fn is_uninitialized(&self) -> bool;

    /// Whether the remover has completed lookups in the current group
    fn is_finished(&self) -> bool;

    /// Whether the group is uninitialized or whether reads are finished
    fn is_uninitialized_or_finished(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::state::{
        bitmap_group::BitmapGroup,
        order::group_position::GroupPosition,
        remove::{GroupPositionRemover, IGroupPositionRemover},
        ArbContext, ContextActions, InnerIndex, OuterIndex, RestingOrderIndex, Side,
    };

    use super::IGroupPositionSequentialRemover;

    #[test]
    fn test_for_asks() {
        let ctx = &mut ArbContext::new();
        let side = Side::Ask;
        let mut remover = GroupPositionRemover::new(side);
        assert_eq!(remover.is_uninitialized(), true);

        let outer_index = OuterIndex::ONE;
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b0000_0101;
        bitmap_group.inner[1] = 0b0000_0010;
        bitmap_group.inner[31] = 0b0000_0001;
        bitmap_group.write_to_slot(ctx, &outer_index);

        remover.load_outer_index(ctx, outer_index);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.is_uninitialized(), false);
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0101);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(2)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0100);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0000);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[1], 0b0000_0000);

        assert_eq!(remover.next(), None);
        assert_eq!(remover.inner.bitmap_group.inner[31], 0b0000_0000);
        assert_eq!(remover.is_finished(), true);
    }

    #[test]
    fn test_for_bids() {
        let ctx = &mut ArbContext::new();
        let side = Side::Bid;
        let mut remover = GroupPositionRemover::new(side);
        assert_eq!(remover.is_uninitialized(), true);

        let outer_index = OuterIndex::ONE;
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b0000_0101;
        bitmap_group.inner[1] = 0b0000_0010;
        bitmap_group.inner[31] = 0b0000_0001;
        bitmap_group.write_to_slot(ctx, &outer_index);
        remover.load_outer_index(ctx, outer_index);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[31], 0b0000_0001);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[31], 0b0000_0000);

        // Resting order indices are looked up from 0, be it bid or ask
        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[1], 0b0000_0000);
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0101);

        assert_eq!(
            remover.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(2)
            }
        );
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0100);

        assert_eq!(remover.next(), None);
        assert_eq!(remover.inner.bitmap_group.inner[0], 0b0000_0000);
        assert_eq!(remover.is_finished(), true);
    }
}
