use super::{ArbContext, ListKey, ListSlot, OuterIndex, Side};

pub fn write_outer_indices_for_tests(
    ctx: &mut ArbContext,
    side: Side,
    outer_indices: Vec<OuterIndex>,
) {
    let slot_count = outer_indices.len() / 16;

    for slot_index in 0..=slot_count {
        let mut list_slot = ListSlot::default();
        let slot_key = ListKey {
            side,
            index: slot_index as u16,
        };

        let end_outer_index_position = outer_indices.len() - slot_index * 16;

        for outer_index_position in 0..end_outer_index_position {
            let inner_slot_index = 16 * slot_index + outer_index_position;
            let outer_index = outer_indices.get(inner_slot_index).unwrap();
            list_slot.set(outer_index_position, *outer_index);
        }
        list_slot.write_to_slot(ctx, &slot_key);
    }
}
