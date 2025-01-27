use crate::state::{order::group_position::GroupPosition, Side};

pub struct GroupPositionIteratorV2 {
    side: Side,
    // initialized: bool,
    current_index: u8,
}

impl GroupPositionIteratorV2 {
    fn current_position(&self) -> GroupPosition {
        GroupPosition::from_index_inclusive(self.side, self.current_index)
    }
}

impl Iterator for GroupPositionIteratorV2 {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == 255 {
            return None;
        }

        self.current_index += 1;
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
