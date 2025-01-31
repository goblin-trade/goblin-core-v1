use crate::state::{order::group_position::GroupPosition, remove::IGroupPositionRemover};

pub trait IGroupPositionLookupRemover: IGroupPositionRemover {
    // Setters

    /// Paginates to the given position and check whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a blank value.
    fn visit_and_check_if_active(&mut self, group_position: GroupPosition) -> bool;

    /// Deactivate the bit at the currently tracked group position
    ///
    /// Externally ensure that try_traverse_to_best_active_position() is called
    /// before deactivation
    fn deactivate_current(&mut self);

    // Getters

    /// Whether `group_position` holds the lowest active bit on its corresponding
    /// inner index (i.e. price).
    ///
    /// Whether the given group position is active and and the lowest resting
    /// order index
    fn is_lowest_resting_order_on_tick(&self, group_position: GroupPosition) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::state::{
        bitmap_group::BitmapGroup,
        order::group_position::GroupPosition,
        remove::{GroupPositionRemover, IGroupPositionRemover, IGroupPositionSequentialRemover},
        ArbContext, ContextActions, InnerIndex, OuterIndex, RestingOrderIndex, Side,
    };

    use super::IGroupPositionLookupRemover;

    mod test_find_and_remove {
        use super::*;

        #[test]
        fn test_find_positions_for_asks() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b0000_0101;
            bitmap_group.inner[31] = 0b1000_0000;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            // Position is active
            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0),
            };
            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.current_position().unwrap(), position_0);

            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                1
            );

            // Position is not active
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(1),
            };
            assert_eq!(remover.visit_and_check_if_active(position_1), false);
            assert_eq!(remover.current_position().unwrap(), position_1);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                2
            );

            // Return to position 0
            assert_eq!(remover.visit_and_check_if_active(position_0), true);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                1
            );

            // Another position that is active
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(2),
            };
            assert_eq!(remover.visit_and_check_if_active(position_2), true);
            assert_eq!(remover.current_position().unwrap(), position_2);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                3
            );

            // Last position
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7),
            };
            assert_eq!(remover.visit_and_check_if_active(position_3), true);
            assert_eq!(remover.current_position().unwrap(), position_3);

            // Exhausted as we navigated to the last item
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                255
            );
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .exhausted,
                true
            );
            assert_eq!(remover.is_exhausted(), true);
        }

        #[test]
        fn test_find_positions_for_bids() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b1000_0000;
            bitmap_group.inner[31] = 0b0000_0101;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            // Position is active
            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0),
            };
            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.current_position().unwrap(), position_0);

            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                1
            );

            // Position is not active
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(1),
            };
            assert_eq!(remover.visit_and_check_if_active(position_1), false);
            assert_eq!(remover.current_position().unwrap(), position_1);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                2
            );

            // Return to position 0
            assert_eq!(remover.visit_and_check_if_active(position_0), true);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                1
            );

            // Another position that is active
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(2),
            };
            assert_eq!(remover.visit_and_check_if_active(position_2), true);
            assert_eq!(remover.current_position().unwrap(), position_2);
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                3
            );

            // Last position
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7),
            };
            assert_eq!(remover.visit_and_check_if_active(position_3), true);
            assert_eq!(remover.current_position().unwrap(), position_3);

            // Exhausted as we navigated to the last item
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .next_index,
                255
            );
            assert_eq!(
                remover
                    .active_group_position_iterator
                    .group_position_iterator
                    .exhausted,
                true
            );
            assert_eq!(remover.is_exhausted(), true);
        }

        #[test]
        fn test_remove_positions_for_asks() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b0000_0101;
            bitmap_group.inner[31] = 0b1000_0000;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0),
            };
            assert_eq!(remover.visit_and_check_if_active(position_0), true);
            remover.deactivate_current();
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0100
            );

            // Removal does not change group_position()
            assert_eq!(remover.current_position().unwrap(), position_0);

            // Last position
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7),
            };
            assert_eq!(remover.visit_and_check_if_active(position_1), true);
            remover.deactivate_current();
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0000
            );
            assert_eq!(remover.current_position().unwrap(), position_1);
        }

        #[test]
        fn test_remove_positions_for_bids() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b1000_0000;
            bitmap_group.inner[31] = 0b0000_0101;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0),
            };
            assert_eq!(remover.visit_and_check_if_active(position_0), true);
            remover.deactivate_current();
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0100
            );

            // Removal does not change group_position()
            assert_eq!(remover.current_position().unwrap(), position_0);

            // Last position
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7),
            };
            assert_eq!(remover.visit_and_check_if_active(position_1), true);
            remover.deactivate_current();
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0000
            );
            assert_eq!(remover.current_position().unwrap(), position_1);
        }
    }

    mod lookup_first_then_sequentially_remove {
        use super::*;

        #[test]
        fn test_lookup_ask_then_sequentially_remove() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b0000_0101;
            bitmap_group.inner[1] = 0b0000_0010;
            bitmap_group.inner[31] = 0b1000_0000;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1),
            };
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7),
            };

            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.next().unwrap(), position_1);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0100
            );

            assert_eq!(remover.next().unwrap(), position_2);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0000
            );

            assert_eq!(remover.next().unwrap(), position_3);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[1],
                0b0000_0000
            );

            assert_eq!(remover.next(), None);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0000
            );
        }

        #[test]
        fn test_lookup_bid_then_sequentially_remove() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b1000_0000;
            bitmap_group.inner[1] = 0b0000_0010;
            bitmap_group.inner[31] = 0b0000_0101;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1),
            };
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7),
            };

            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.next().unwrap(), position_1);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0100
            );

            assert_eq!(remover.next().unwrap(), position_2);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0000
            );

            assert_eq!(remover.next().unwrap(), position_3);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[1],
                0b0000_0000
            );

            assert_eq!(remover.next(), None);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0000
            );
        }

        #[test]
        fn test_lookup_ask_sequentially_remove_then_lookup_again() {
            let ctx = &mut ArbContext::new();
            let side = Side::Ask;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b0000_0101;
            bitmap_group.inner[1] = 0b0000_0010;
            bitmap_group.inner[31] = 0b1000_0000;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1),
            };
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(7),
            };

            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.next().unwrap(), position_1);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0100
            );

            assert_eq!(remover.next().unwrap(), position_2);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b0000_0000
            );

            // skip removing position_2, lookup position 3
            assert_eq!(remover.visit_and_check_if_active(position_3), true);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[1],
                0b0000_0010
            );
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b1000_0000
            );
        }

        #[test]
        fn test_lookup_bid_sequentially_remove_then_lookup_again() {
            let ctx = &mut ArbContext::new();
            let side = Side::Bid;
            let mut remover = GroupPositionRemover::new(side);

            let outer_index = OuterIndex::ONE;
            let mut bitmap_group = BitmapGroup::default();
            bitmap_group.inner[0] = 0b1000_0000;
            bitmap_group.inner[1] = 0b0000_0010;
            bitmap_group.inner[31] = 0b0000_0101;
            bitmap_group.write_to_slot(ctx, &outer_index);
            remover.load_outer_index(ctx, outer_index);

            let position_0 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(0),
            };
            let position_1 = GroupPosition {
                inner_index: InnerIndex::new(31),
                resting_order_index: RestingOrderIndex::new(2),
            };
            let position_2 = GroupPosition {
                inner_index: InnerIndex::new(1),
                resting_order_index: RestingOrderIndex::new(1),
            };
            let position_3 = GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(7),
            };

            assert_eq!(remover.visit_and_check_if_active(position_0), true);

            assert_eq!(remover.next().unwrap(), position_1);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0100
            );

            assert_eq!(remover.next().unwrap(), position_2);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[31],
                0b0000_0000
            );

            // skip removing position_2, lookup position 3
            assert_eq!(remover.visit_and_check_if_active(position_3), true);
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[1],
                0b0000_0010
            );
            assert_eq!(
                remover.active_group_position_iterator.bitmap_group.inner[0],
                0b1000_0000
            );
        }
    }
}
