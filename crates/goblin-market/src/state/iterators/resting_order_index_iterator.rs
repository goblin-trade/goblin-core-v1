use crate::state::RestingOrderIndex;

/// Loop from the min (0) to max (7) value of RestingOrderIndex
pub struct RestingOrderIndexIterator {
    /// Begin lookup one position ahead of `last_index`.
    /// This acts as an index for lookups.
    last_index: Option<RestingOrderIndex>,
}

impl RestingOrderIndexIterator {
    /// Initializes a new RestingOrderIndexIterator
    ///
    /// # Arguments
    ///
    /// * `last_index` - The last position to exclude while looping
    ///
    pub fn new(last_index: Option<RestingOrderIndex>) -> Self {
        RestingOrderIndexIterator { last_index }
    }
}

impl Iterator for RestingOrderIndexIterator {
    type Item = RestingOrderIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_index) = self.last_index {
            if last_index == RestingOrderIndex::MAX {
                return None; // Reached the end
            }
            self.last_index = Some(last_index + RestingOrderIndex::ONE);
        } else {
            self.last_index = Some(RestingOrderIndex::ZERO);
        }
        self.last_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resting_order_index_iterator_start_none() {
        // last_index = None
        let mut iter = RestingOrderIndexIterator::new(None);

        // Loop through values from 0 to 7
        for i in 0..=RestingOrderIndex::MAX.as_u8() {
            assert_eq!(iter.next(), Some(RestingOrderIndex::new(i)));
        }

        // After reaching MAX (7), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resting_order_index_iterator_start_zero() {
        // last_index = Some(RestingOrderIndex::ZERO)
        let mut iter = RestingOrderIndexIterator::new(Some(RestingOrderIndex::ZERO));

        // Loop through values from 1 to 7
        for i in 1..=RestingOrderIndex::MAX.as_u8() {
            assert_eq!(iter.next(), Some(RestingOrderIndex::new(i)));
        }

        // After reaching MAX (7), next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_resting_order_index_iterator_start_max() {
        // last_index = Some(RestingOrderIndex::MAX)
        let mut iter = RestingOrderIndexIterator::new(Some(RestingOrderIndex::MAX));

        // Since last_index is already MAX (7), next should return None immediately
        assert_eq!(iter.next(), None);
    }
}
