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
pub struct BitIndexIterator {
    pub current_index: Option<u8>,
}

impl BitIndexIterator {
    pub fn current_index(&self) -> Option<u8> {
        self.current_index
    }

    pub fn set_current_index(&mut self, index: Option<u8>) {
        self.current_index = index;
    }

    pub fn peek(&self) -> Option<u8> {
        match self.current_index {
            None => Some(0),        // Will start at 0
            Some(255) => None,      // Already at end
            Some(i) => Some(i + 1), // Next value will be current + 1
        }
    }
}

impl Iterator for BitIndexIterator {
    type Item = u8;

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
        let mut iter = BitIndexIterator {
            current_index: None,
        };
        assert_eq!(
            iter.current_index(),
            None,
            "Fresh iterator should have no current index"
        );

        // Test first value
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.current_index(), Some(0));

        // Test sequential values up to 253
        for i in 0..253 {
            assert_eq!(iter.next(), Some(i + 1));
            assert_eq!(iter.current_index(), Some(i + 1));
        }

        // Test boundary transition (254 -> 255 -> None)
        assert_eq!(iter.next(), Some(254));
        assert_eq!(iter.current_index(), Some(254));

        assert_eq!(iter.next(), Some(255));
        assert_eq!(iter.current_index(), Some(255));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.current_index(), Some(255));

        // Test set_current_index
        // Set to middle value
        iter.set_current_index(Some(100));
        assert_eq!(iter.current_index(), Some(100));
        assert_eq!(iter.next(), Some(101));

        // Set to last value
        iter.set_current_index(Some(255));
        assert_eq!(iter.current_index(), Some(255));
        assert_eq!(iter.next(), None);

        // Set back to start
        iter.set_current_index(Some(0));
        assert_eq!(iter.current_index(), Some(0));
        assert_eq!(iter.next(), Some(1));

        // Create new iterator and collect all values
        let mut iter = BitIndexIterator {
            current_index: None,
        };
        let values: Vec<u8> = iter.collect();
        assert_eq!(values.len(), 256, "Should yield exactly 256 values");
        for i in 0..256 {
            assert_eq!(values[i], i as u8, "Values should be sequential");
        }
    }
}
