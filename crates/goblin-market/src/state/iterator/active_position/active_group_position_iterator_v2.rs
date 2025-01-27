use crate::state::{
    bitmap_group::BitmapGroup,
    iterator::position::group_position_iterator_v2::GroupPositionIteratorV2,
    order::group_position::GroupPosition, Side,
};

/// Iterator to find coordinates of active bits in a bitmap group
pub struct ActiveGroupPositionIteratorV2 {
    /// The bitmap group to search
    pub bitmap_group: BitmapGroup,

    /// Iterator to obtain bitmap group coordinates
    pub group_position_iterator: GroupPositionIteratorV2,
}

impl ActiveGroupPositionIteratorV2 {
    pub fn new(bitmap_group: BitmapGroup, side: Side, index: u8) -> Self {
        ActiveGroupPositionIteratorV2 {
            bitmap_group,
            group_position_iterator: GroupPositionIteratorV2::new(side, index),
        }
    }

    pub fn new_default_for_side(side: Side) -> Self {
        ActiveGroupPositionIteratorV2 {
            bitmap_group: BitmapGroup::default(),

            // When a new active iterator is initialized, we are already at index 0
            // Calling next will take us to 1 or a subsequent position
            // Calling current_index()- we can't assume that the 0th index holds an
            // active bit. First test for 0, if it is absent then loop with next()
            group_position_iterator: GroupPositionIteratorV2::new(side, 0),
        }
    }

    pub fn side(&self) -> Side {
        self.group_position_iterator.side
    }

    pub fn new_with_starting_position(
        bitmap_group: BitmapGroup,
        side: Side,
        starting_position_inclusive: GroupPosition,
    ) -> Self {
        let count = starting_position_inclusive.index_inclusive(side);
        ActiveGroupPositionIteratorV2::new(bitmap_group, side, count)
    }

    // Lookup remover functions

    /// Paginates to the given position and check whether the bit is active
    pub fn find(&mut self, group_position: GroupPosition) -> bool {
        self.group_position_iterator.set_position(group_position);
        self.bitmap_group.is_position_active(group_position)
    }

    /// The group position looked up by find()
    pub fn current_position(&self) -> GroupPosition {
        self.group_position_iterator.current_position()
    }
}

impl Iterator for ActiveGroupPositionIteratorV2 {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        // Special case for 0th index
        // Don't call next() otherwise 0 will never be tested.
        // Instead test for 0, if bit is not active then move on to other positions
        if self.group_position_iterator.current_index == 0 {
            let position = self.group_position_iterator.current_position();
            if self.bitmap_group.is_position_active(position) {
                self.group_position_iterator.increment_index();

                return Some(position);
            }
        }

        while let Some(group_position) = self.group_position_iterator.next() {
            if self.bitmap_group.is_position_active(group_position) {
                return Some(group_position);
            }
        }

        None
    }
}
