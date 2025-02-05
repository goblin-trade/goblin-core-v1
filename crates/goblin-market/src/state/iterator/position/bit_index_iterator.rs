use crate::state::BitIndex;

/// Iterate through bit indices in a bitmap group
///
/// * This is agnostic to side. The direction of traversing across
/// inner indices and resting order indices is adjusted externally for sides.
///
/// * For example for bids
///   * i = 0, inner_index = 31, resting_order_index = 0
///   * i = 1, inner_index = 31, resting_order_index = 1, and so on
///
/// * This iterator tracks the **current** bit index, not next.
///
/// * It has 256 + 1 states. State None means that the iterator is uninitialized.
/// States Some(n) represent current values from 0 to 255.
///
/// TODO custom BitIndex type. Conversions inside group_position are dicey
#[derive(Default)]
pub struct BitIndexIterator {
    pub current_index: Option<BitIndex>,
}

impl BitIndexIterator {
    pub fn set_current_index(&mut self, index: Option<BitIndex>) {
        self.current_index = index;
    }

    pub fn peek(&self) -> Option<BitIndex> {
        match self.current_index {
            None => Some(BitIndex::ZERO),       // Will start at 0
            Some(BitIndex::MAX) => None,        // Already at end
            Some(i) => Some(i + BitIndex::ONE), // Next value will be current + 1
        }
    }
}

impl Iterator for BitIndexIterator {
    type Item = BitIndex;

    fn next(&mut self) -> Option<Self::Item> {
        // Get next value using peek
        let next = self.peek()?;
        // Update state
        self.current_index = Some(next);
        // Return value
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_index_iterator() {
        // Test initial state
        let mut iter = BitIndexIterator::default();
        assert_eq!(
            iter.current_index, None,
            "Fresh iterator should have no current index"
        );

        // Test first value
        assert_eq!(iter.next().unwrap().as_u8(), 0);
        assert_eq!(iter.current_index.unwrap().as_u8(), 0);

        // Test sequential values up to 253
        for i in 0..253 {
            assert_eq!(iter.next().unwrap().as_u8(), i + 1);
            assert_eq!(iter.current_index.unwrap().as_u8(), i + 1);
        }

        // TODO fix rest of tests. Use unwrap().as_u8() and remove Some()

        // Test boundary transition (254 -> 255 -> None)
        assert_eq!(iter.next().unwrap().as_u8(), 254);
        assert_eq!(iter.current_index.unwrap().as_u8(), 254);

        assert_eq!(iter.next().unwrap().as_u8(), 255);
        assert_eq!(iter.current_index.unwrap().as_u8(), 255);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.current_index.unwrap().as_u8(), 255);

        // Test set_current_index
        // Set to middle value
        iter.set_current_index(Some(BitIndex::new(100)));
        assert_eq!(iter.current_index.unwrap().as_u8(), 100);
        assert_eq!(iter.next().unwrap().as_u8(), 101);

        // Set to last value
        iter.set_current_index(Some(BitIndex::new(255)));
        assert_eq!(iter.current_index.unwrap().as_u8(), 255);
        assert_eq!(iter.next(), None);

        // Set back to start
        iter.set_current_index(Some(BitIndex::new(0)));
        assert_eq!(iter.current_index.unwrap().as_u8(), 0);
        assert_eq!(iter.next().unwrap().as_u8(), 1);

        // Create new iterator and collect all values
        let iter = BitIndexIterator::default();
        let values: Vec<BitIndex> = iter.collect();
        assert_eq!(values.len(), 256, "Should yield exactly 256 values");
        for i in 0..=255 {
            assert_eq!(
                values[i],
                BitIndex::new(i as u8),
                "Values should be sequential"
            );
        }
    }
}
