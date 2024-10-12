use crate::state::{order::group_position::GroupPosition, remove::IGroupPositionRemover};

pub trait IGroupPositionSequentialRemover: IGroupPositionRemover {
    /// Deactivate current position and get next
    fn deactivate_current_and_get_next(&mut self) -> Option<GroupPosition>;

    /// Whether the group is uninitialized or whether reads are finished
    fn is_uninitialized_or_finished(&self) -> bool;
}
