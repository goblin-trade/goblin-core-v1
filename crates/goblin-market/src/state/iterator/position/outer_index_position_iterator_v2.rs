/// Coordinates of an outer index stored in the index list
/// The index list consists of ordered outer indices stored in `list_slots`.
/// Each `list_slot` stores upto 16 values.
///
/// The position of a stored outer index can be given as coordinates
/// (slot_index, relative_index)
///
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct OuterIndexPosition {
    /// Index of the slot
    pub slot_index: u16,

    /// Relative index (0 to 15) within the slot
    pub relative_index: u8,
}

/// Iterator to get coordinates of stored outer indices from the
/// end of the list to the start.
///
/// Used to read active outer indices in active_position/outer_index
///
pub struct OuterIndexPositionIteratorV2<'a> {
    /// Number of indices yet to be read
    pub outer_index_count: &'a mut u16,
}

impl<'a> OuterIndexPositionIteratorV2<'a> {
    pub fn slot_index(&self) -> u16 {
        (*self.outer_index_count - 1) / 16
    }

    pub fn relative_index(&self) -> u8 {
        ((*self.outer_index_count - 1) % 16) as u8
    }

    pub fn outer_index_position(&self) -> OuterIndexPosition {
        OuterIndexPosition {
            slot_index: self.slot_index(),
            relative_index: self.relative_index(),
        }
    }
}

impl<'a> Iterator for OuterIndexPositionIteratorV2<'a> {
    type Item = OuterIndexPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.outer_index_count == 0 {
            return None;
        }
        let result = Some(self.outer_index_position());
        *self.outer_index_count -= 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indices_across_two_slots() {
        let mut outer_index_count = 17;

        let mut iterator = OuterIndexPositionIteratorV2 {
            outer_index_count: &mut outer_index_count,
        };

        assert_eq!(
            iterator.next().unwrap(),
            OuterIndexPosition {
                slot_index: 1,
                relative_index: 0
            }
        );
        for i in (0..=15).rev() {
            assert_eq!(
                iterator.next().unwrap(),
                OuterIndexPosition {
                    slot_index: 0,
                    relative_index: i
                }
            );
        }
        assert!(iterator.next().is_none());
    }
}
