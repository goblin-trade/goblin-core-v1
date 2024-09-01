Write tests
1. Deactivate on a blank bitmap group, i.e. bit is already off

2. Deactivate single order

3. Deactivate two orders on same tick

4. Deactivate two orders on different inner indices on the same bitmap group

5. Deactivate two orders on different bitmap groups

A note on terminology
- A tick is made up of an outer index and an inner index
- Outer index corresponds to a 256 bit bitmap group
- Each bitmap group has 32 bitmaps of 8 bit each
- Each bitmap is denoted by inner index
- Each bit inside a bitmap is denoted by resting_order_index
- Together (outer_index, inner_index, resting_order_index) tell whether an order is present at a price.
- A single price tick can hold at the most 8 orders

Refer to these BitmapInserter tests

```rs
#[cfg(test)]
mod tests {
    use crate::{
        quantities::{Ticks, WrapperU64},
        state::{BitmapGroup, OrderId, RestingOrderIndex, SlotActions, SlotStorage},
    };

    use super::BitmapInserter;

    #[test]
    fn insert_single_order() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id.price_in_ticks.outer_index();

        // 1. Activate and check
        inserter.activate(slot_storage, &order_id, bitmap_group_is_empty);
        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_same_tick() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(1),
        };

        let outer_index = order_id_0.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty);

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000011;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_at_different_inner_indices_on_same_group() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::ONE,
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index = order_id_0.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty);

        assert_eq!(outer_index, inserter.last_outer_index.unwrap());

        let mut expected_bitmap_group = BitmapGroup::default();
        expected_bitmap_group.inner[0] = 0b00000001;
        expected_bitmap_group.inner[1] = 0b00000001;
        assert_eq!(expected_bitmap_group, inserter.bitmap_group);

        let mut read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(BitmapGroup::default(), read_bitmap_group);

        // 2. Write to slot and check
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);
        assert_eq!(expected_bitmap_group, read_bitmap_group);
    }

    #[test]
    fn insert_two_orders_in_different_groups() {
        let slot_storage = &mut SlotStorage::new();
        let bitmap_group_is_empty = true;

        let mut inserter = BitmapInserter::new();

        let order_id_0 = OrderId {
            price_in_ticks: Ticks::ZERO,
            resting_order_index: RestingOrderIndex::new(0),
        };
        let order_id_1 = OrderId {
            price_in_ticks: Ticks::new(32),
            resting_order_index: RestingOrderIndex::new(0),
        };

        let outer_index_0 = order_id_0.price_in_ticks.outer_index();
        let outer_index_1 = order_id_1.price_in_ticks.outer_index();

        // 1. Activate
        inserter.activate(slot_storage, &order_id_0, bitmap_group_is_empty);
        inserter.activate(slot_storage, &order_id_1, bitmap_group_is_empty); // this will write first group

        let mut expected_bitmap_group_0 = BitmapGroup::default();
        expected_bitmap_group_0.inner[0] = 0b00000001;
        let expected_bitmap_group_1 = expected_bitmap_group_0;

        // bitmap_group_0 has been written to slot
        let read_bitmap_group_0 = BitmapGroup::new_from_slot(slot_storage, outer_index_0);
        assert_eq!(expected_bitmap_group_0, read_bitmap_group_0);

        // bitmap_group_1 and outer_index_1 are still in cache
        let mut read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(BitmapGroup::default(), read_bitmap_group_1);
        assert_eq!(outer_index_1, inserter.last_outer_index.unwrap());
        assert_eq!(expected_bitmap_group_1, inserter.bitmap_group);

        // 2. Write cache
        inserter.write_last_bitmap_group(slot_storage);
        read_bitmap_group_1 = BitmapGroup::new_from_slot(slot_storage, outer_index_1);
        assert_eq!(expected_bitmap_group_1, read_bitmap_group_1);
    }
}
```
