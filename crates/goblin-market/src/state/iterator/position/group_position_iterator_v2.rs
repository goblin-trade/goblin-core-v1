use core::{iter::Peekable, ops::RangeInclusive};

use crate::state::{order::group_position::GroupPosition, Side};

pub struct GroupPositionIteratorV2 {
    pub side: Side,
    pub inner: Peekable<RangeInclusive<u8>>,
}

impl GroupPositionIteratorV2 {
    pub fn new(side: Side, start_index_inclusive: u8) -> Self {
        GroupPositionIteratorV2 {
            side,
            inner: (start_index_inclusive..=255).peekable(),
        }
    }

    // used by lookup remover to jump to a group position
    pub fn set_current_position(&mut self, group_position_inclusive: GroupPosition) {
        let group_position_index = group_position_inclusive.index_inclusive(self.side);
        let range_start = group_position_index.wrapping_add(1);

        self.inner = (range_start..=255).peekable();
    }

    // peek() function requires mutable reference. This operator does not
    // iteratate to the next element
    pub fn current_index(&mut self) -> u8 {
        if let Some(peeked) = self.inner.peek() {
            let index = peeked.wrapping_sub(1);

            index
        } else {
            255
        }
    }

    pub fn group_position(&mut self) -> GroupPosition {
        GroupPosition::from_index_inclusive(self.side, self.current_index())
    }
}

impl Iterator for GroupPositionIteratorV2 {
    type Item = GroupPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.group_position();

        if result.is_some() {
            self.index = self.index.wrapping_add(1);
            self.finished = self.index == 0;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use core::{
        iter::Peekable,
        ops::{Range, RangeInclusive},
    };

    use super::*;

    #[test]
    fn wrapped_sub() {
        let mut i: u8 = 0;
        i = i.wrapping_sub(1);

        println!("wrapped sub result {}", i);
    }

    #[test]
    fn test_range_start_for_none() {
        let range_start = 255;
        let mut range: RangeInclusive<u8> = range_start..=255;
        range.next();

        // let mut gg = RangeInclusive {
        //     start: 255,
        //     end: 255,
        //     exhausted: true,
        // };
    }

    #[test]
    fn test_iterator() {
        let mut range: Peekable<RangeInclusive<u8>> = (0..=255).peekable();

        // println!("upcoming value {:?}", range.peek());

        // let next_value = range.next();
        // println!("next_value {:?}", next_value);
    }
}
