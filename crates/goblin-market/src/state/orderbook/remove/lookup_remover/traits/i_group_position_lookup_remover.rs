use crate::state::{order::group_position::GroupPosition, remove::IGroupPositionRemover};

pub trait IGroupPositionLookupRemover: IGroupPositionRemover {
    // Setters

    /// Paginates to the given position and check whether the bit is active
    ///
    /// Externally ensure that load_outer_index() was called first otherwise
    /// this will give a blank value.
    fn paginate_and_check_if_active(&mut self, group_position: GroupPosition) -> bool;

    /// Deactivate the bit at the currently tracked group position
    ///
    /// Externally ensure that try_traverse_to_best_active_position() is called
    /// before deactivation
    fn deactivate(&mut self, group_position: GroupPosition);

    // Getters

    /// Whether `group_position` holds the only active bit on its corresponding
    /// inner index and by extension price
    fn is_only_active_bit_on_tick(&self, group_position: GroupPosition) -> bool;

    /// Whether the current bitmap group has any active positions
    fn is_group_active(&self) -> bool;
}
