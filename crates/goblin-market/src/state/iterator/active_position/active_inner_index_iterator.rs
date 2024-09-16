use crate::state::{
    bitmap_group::BitmapGroup, iterator::position::inner_index_iterator::InnerIndexIterator,
    InnerIndex, Side,
};

/// Iterates through active inner indices in a bitmap group
pub struct ActiveInnerIndexIterator<'a> {
    /// The bitmap group to lookup
    bitmap_group: &'a BitmapGroup,

    /// Iterator for inner indices. We can begin lookup from a specific index
    inner: InnerIndexIterator,
}

impl<'a> ActiveInnerIndexIterator<'a> {
    /// Initializes a new `ActiveInnerIndexIterator`
    ///
    /// # Arguments
    ///
    /// * `bitmap_group`
    /// * `side`
    /// * `starting_index` - If Some, begin lookup beginning from this index (inclusive)
    ///
    pub fn new(
        bitmap_group: &'a BitmapGroup,
        side: Side,
        starting_index: Option<InnerIndex>,
    ) -> Self {
        ActiveInnerIndexIterator {
            bitmap_group,
            inner: InnerIndexIterator::new_with_starting_index(side, starting_index),
        }
    }
}

impl<'a> Iterator for ActiveInnerIndexIterator<'a> {
    type Item = InnerIndex;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(inner_index) = self.inner.next() {
            if self.bitmap_group.inner_index_is_active(inner_index) {
                return Some(inner_index);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InnerIndex, Side};

    #[test]
    fn test_active_indices_in_asks_from_start() {
        let mut bitmap_group = BitmapGroup::default();

        let active_inner_indices = vec![0, 1, 10, 31];

        for active_inner_index in &active_inner_indices {
            bitmap_group.inner[*active_inner_index] = 1;
        }

        let starting_index = None;
        let side = Side::Ask;
        let mut iterator = ActiveInnerIndexIterator::new(&bitmap_group, side, starting_index);

        assert_eq!(iterator.next().unwrap(), InnerIndex::new(0));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(1));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(10));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(31));
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_active_indices_in_bids_from_start() {
        let mut bitmap_group = BitmapGroup::default();

        let active_inner_indices = vec![0, 1, 10, 31];

        for active_inner_index in &active_inner_indices {
            bitmap_group.inner[*active_inner_index] = 1;
        }

        let starting_index = None;
        let side = Side::Bid;
        let mut iterator = ActiveInnerIndexIterator::new(&bitmap_group, side, starting_index);

        assert_eq!(iterator.next().unwrap(), InnerIndex::new(31));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(10));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(1));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(0));
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_active_indices_in_asks_from_given_index() {
        let mut bitmap_group = BitmapGroup::default();

        let active_inner_indices = vec![0, 1, 10, 31];

        for active_inner_index in &active_inner_indices {
            bitmap_group.inner[*active_inner_index] = 1;
        }

        let starting_index = Some(InnerIndex::new(1));
        let side = Side::Ask;
        let mut iterator = ActiveInnerIndexIterator::new(&bitmap_group, side, starting_index);

        // Index 0 is skipped
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(1));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(10));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(31));
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_active_indices_in_bids_from_given_index() {
        let mut bitmap_group = BitmapGroup::default();

        let active_inner_indices = vec![0, 1, 10, 31];

        for active_inner_index in &active_inner_indices {
            bitmap_group.inner[*active_inner_index] = 1;
        }

        let starting_index = Some(InnerIndex::new(10));
        let side = Side::Bid;
        let mut iterator = ActiveInnerIndexIterator::new(&bitmap_group, side, starting_index);

        assert_eq!(iterator.next().unwrap(), InnerIndex::new(10));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(1));
        assert_eq!(iterator.next().unwrap(), InnerIndex::new(0));
        assert!(iterator.next().is_none());
    }
}
