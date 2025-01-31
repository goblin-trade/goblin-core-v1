use crate::state::{order::group_position::GroupPosition, Side};

pub struct GroupPositionIteratorV2 {
    pub side: Side,
    // initialized: bool,
    pub current_index: u8,
}

impl GroupPositionIteratorV2 {
    pub fn new(side: Side, current_index: u8) -> Self {
        GroupPositionIteratorV2 {
            side,
            current_index,
        }
    }

    pub fn increment_index(&mut self) {
        debug_assert!(self.current_index < 255);
        self.current_index += 1;
    }

    pub fn current_position(&self) -> GroupPosition {
        GroupPosition::from_bit_index(self.side, self.current_index)
    }

    pub fn set_position(&mut self, position: GroupPosition) {
        let new_index = position.bit_index(self.side);
        self.current_index = new_index;
    }
}

impl Iterator for GroupPositionIteratorV2 {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == 255 {
            return None;
        }
        self.increment_index();
        Some(self.current_position())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_range() {
        let mut my_range = 0u8..=255;

        let next_val = my_range.next();

        // There's no way to get the current value
    }
}
