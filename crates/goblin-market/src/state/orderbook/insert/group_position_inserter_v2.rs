use crate::state::{
    bitmap_group::BitmapGroup, order::group_position::GroupPosition, ArbContext, OuterIndex,
};

/// Facilitates efficient batch activations in bitmap groups
///
/// Unlike GroupPositionRemover, we use an inner BitmapGroup instead of an
/// ActivePositionIterator because iterations are not needed.
pub struct GroupPositionInserterV2 {
    /// The current bitmap group pending a write. This allows us to perform multiple
    /// updates in a bitmap group with a single slot load. This value is written to slot
    /// when outer index changes or when writes are complete.
    pub bitmap_group: BitmapGroup,
}

impl GroupPositionInserterV2 {
    pub fn new() -> Self {
        GroupPositionInserterV2 {
            bitmap_group: BitmapGroup::default(),
        }
    }

    /// Load bitmap group for the given outer index
    ///
    /// If outer index is known to be closed, use load_empty_group() instead
    /// to avoid wasting gas on an SLOAD.
    pub fn load_outer_index(&mut self, ctx: ArbContext, outer_index: OuterIndex) {
        self.bitmap_group = BitmapGroup::new_from_slot(&ctx, outer_index);
    }

    /// Load an empty bitmap group
    pub fn load_empty_group(&mut self) {
        self.bitmap_group = BitmapGroup::default();
    }

    /// Activate an order in the current bitmap group at the given GroupPosition
    pub fn activate(&mut self, group_position: GroupPosition) {
        self.bitmap_group.activate(group_position);
    }

    // TODO write_to_slot() and clear_garbage_bits()
}
