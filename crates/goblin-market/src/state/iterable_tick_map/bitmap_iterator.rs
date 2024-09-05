use crate::state::{
    InnerIndex, InnerIndexIterator, RestingOrderIndex, RestingOrderIndexIterator, Side,
};

use super::BitmapGroup;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupPosition {
    pub inner_index: InnerIndex,
    pub resting_order_index: RestingOrderIndex,
}

pub struct BitmapGroupIterator<'a> {
    bitmap_group: &'a BitmapGroup,
    side: Side,
    last_position: Option<GroupPosition>,
}

impl<'a> BitmapGroupIterator<'a> {
    /// Create a new iterator for the given bitmap group, side, and starting position to exclude.
    pub fn new(
        bitmap_group: &'a BitmapGroup,
        side: Side,
        last_position: Option<GroupPosition>,
    ) -> Self {
        BitmapGroupIterator {
            bitmap_group,
            side,
            last_position,
        }
    }
}

impl<'a> Iterator for BitmapGroupIterator<'a> {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner_index_iterator =
            InnerIndexIterator::new(self.side, self.last_position.map(|o| o.inner_index));

        while let Some(inner_index) = inner_index_iterator.next() {
            let bitmap = self.bitmap_group.get_bitmap(&inner_index);
            let mut resting_order_index_iterator =
                RestingOrderIndexIterator::new(self.last_position.map(|o| o.resting_order_index));

            while let Some(resting_order_index) = resting_order_index_iterator.next() {
                if bitmap.order_present(resting_order_index) {
                    #[cfg(test)]
                    println!(
                        "got active position inner_index {:?} resting_order_index {:?}",
                        inner_index, resting_order_index
                    );

                    self.last_position = Some(GroupPosition {
                        inner_index,
                        resting_order_index,
                    });

                    return self.last_position;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_bitmap_group() -> BitmapGroup {
        let mut bitmap_group = BitmapGroup::default();
        // Same bitmap- consecutive and non consecutive
        bitmap_group.inner[0] = 0b10000011;
        // Consecutive bitmap
        bitmap_group.inner[1] = 0b00000001;
        // Non consecutive bitmap
        bitmap_group.inner[3] = 0b00000001;

        //
        bitmap_group.inner[31] = 0b00001010;
        bitmap_group
    }

    #[test]
    fn test_bitmap_group_iterator_same_bitmap_no_last_position() {
        let mut bitmap_group = BitmapGroup::default();
        bitmap_group.inner[0] = 0b10000011;

        let side = Side::Ask;
        let last_position = None;

        let mut iterator = BitmapGroupIterator::new(&bitmap_group, side, last_position);
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(0)
            }
        );
        // Breaks because inner index is skipped?
        assert_eq!(
            iterator.next().unwrap(),
            GroupPosition {
                inner_index: InnerIndex::new(0),
                resting_order_index: RestingOrderIndex::new(1)
            }
        );
        // assert_eq!(
        //     iterator.next().unwrap(),
        //     GroupPosition {
        //         inner_index: InnerIndex::new(0),
        //         resting_order_index: RestingOrderIndex::new(7)
        //     }
        // );
        // assert!(iterator.next().is_none());
    }
}
