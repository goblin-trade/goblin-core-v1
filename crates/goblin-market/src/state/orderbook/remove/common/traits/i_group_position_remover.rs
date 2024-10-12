use crate::state::{order::group_position::GroupPosition, ArbContext, OuterIndex, Side};

pub trait IGroupPositionRemover {
    /// Load bitmap group for the given outer index
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context to read from slot
    /// * `outer_index` - Load bitmap group for this outer index
    fn load_outer_index(&mut self, ctx: &mut ArbContext, outer_index: OuterIndex);

    /// Write the bitmap group to slot
    fn write_to_slot(&self, ctx: &mut ArbContext, outer_index: OuterIndex);

    /// Get side for this remover
    fn side(&self) -> Side;

    /// Get the current group position if it is loaded
    fn group_position(&self) -> Option<GroupPosition>;
}
