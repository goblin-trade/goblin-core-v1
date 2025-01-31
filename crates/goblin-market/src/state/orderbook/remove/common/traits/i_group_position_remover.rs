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

    /// The group position that was looked up
    fn current_position(&self) -> Option<GroupPosition>;

    /// Return a mutable reference to the bitmap group
    fn bitmap_group_mut(&mut self) -> &mut BitmapGroup;

    /// Get side for this remover
    fn side(&self) -> Side;
}
