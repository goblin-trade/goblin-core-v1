use crate::{
    quantities::Ticks,
    state::{
        bitmap_group::BitmapGroup, order::group_position::GroupPosition, ArbContext, OuterIndex,
        Side,
    },
};

/// Common trait shared by both IGroupPositionSequentialRemover and IGroupPositionLookupRemover
pub trait IGroupPositionRemover {
    /// Load bitmap group for the given outer index
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context to read from slot
    /// * `outer_index` - Load bitmap group for this outer index
    fn load_outer_index(&mut self, ctx: &ArbContext, outer_index: OuterIndex);

    /// Get the current bitmap group
    fn get_bitmap_group(&self) -> BitmapGroup;

    /// The group position that was looked up
    fn current_position(&self) -> Option<GroupPosition>;

    /// Externally set the bitmap group
    fn set_bitmap_group(&mut self, bitmap_group: BitmapGroup);

    /// Write the bitmap group to slot
    fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex);

    /// Get side for this remover
    fn side(&self) -> Side;
}
