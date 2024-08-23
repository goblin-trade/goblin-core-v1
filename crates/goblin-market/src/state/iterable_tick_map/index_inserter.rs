use super::{IndexListIterator, ListKey, ListSlot, OuterIndex};
use crate::state::{Side, SlotStorage};
use alloc::vec::Vec;

pub struct IndexListInsertion<'a> {
    pub index_list_iterator: IndexListIterator<'a>,
    pub cache: Vec<OuterIndex>, // Cache for storing outer indices
    pub side: Side,             // Indicates whether it's for Bids or Asks
}

impl<'a> IndexListInsertion<'a> {
    pub fn new(side: Side, outer_index_count: u16, slot_storage: &'a mut SlotStorage) -> Self {
        let index_list_iterator = IndexListIterator::new(outer_index_count, slot_storage);

        Self {
            index_list_iterator,
            cache: Vec::new(),
            side,
        }
    }

    pub fn prepare(&mut self, outer_index: OuterIndex) -> bool {
        // Check last element in the cache
        if let Some(&last_pushed_outer_index) = self.cache.last() {
            // If the element already exists in the cache, return false
            if last_pushed_outer_index == outer_index {
                return false;
            }

            // If the last element in cache is closer to the center than outer_index, insert before it
            if self.is_closer_to_center(last_pushed_outer_index, outer_index) {
                self.cache.pop(); // Remove the last pushed index
                self.cache.push(outer_index);
                self.cache.push(last_pushed_outer_index); // Push it back after the new index
                return true;
            }
        }

        // Iterate through the list to find the correct position
        while let Some((_slot_index, _relative_index, _list_slot, current_outer_index)) =
            self.index_list_iterator.next()
        {
            // If the outer_index is already in the list, ignore it
            if current_outer_index == outer_index {
                return false;
            }

            // If outer_index is closer to the center, insert before current_outer_index
            if self.is_closer_to_center(current_outer_index, outer_index) {
                self.cache.push(outer_index);
                self.cache.push(current_outer_index);
                return true;
            } else {
                // Otherwise, push the current_outer_index to cache and continue
                self.cache.push(current_outer_index);
            }
        }

        // If reached end without finding a suitable position, push the outer_index to cache
        self.cache.push(outer_index);
        true
    }

    fn is_closer_to_center(
        &self,
        current_outer_index: OuterIndex,
        new_outer_index: OuterIndex,
    ) -> bool {
        match self.side {
            Side::Bid => new_outer_index > current_outer_index,
            Side::Ask => new_outer_index < current_outer_index,
        }
    }

    pub fn commit(&mut self) {
        let mut current_slot_index = self.index_list_iterator.slot_index;
        let mut current_relative_index = self.index_list_iterator.relative_index;

        let mut current_list_slot =
            self.index_list_iterator
                .list_slot
                .take()
                .unwrap_or_else(|| {
                    ListSlot::new_from_slot(
                        self.index_list_iterator.slot_storage,
                        ListKey {
                            index: current_slot_index as u16,
                        },
                    )
                });

        for &outer_index in self.cache.iter().rev() {
            // Set the value in the current list slot
            current_list_slot.set(current_relative_index as usize, outer_index);

            // Move to the next index position
            if current_relative_index == 0 {
                // Write the current slot to storage
                current_list_slot.write_to_slot(
                    self.index_list_iterator.slot_storage,
                    &ListKey {
                        index: current_slot_index as u16,
                    },
                );

                // Move to the previous slot
                current_slot_index -= 1;
                current_relative_index = 15;

                // Load or create a new slot
                current_list_slot = ListSlot::new();
            } else {
                current_relative_index -= 1;
            }
        }

        // Write the last slot if needed
        current_list_slot.write_to_slot(
            self.index_list_iterator.slot_storage,
            &ListKey {
                index: current_slot_index as u16,
            },
        );

        // Clear the cache
        self.cache.clear();
    }
}
