use super::sequential_order_remover::SequentialOrderRemover;

pub struct RandomOrderRemover<'a> {
    inner: SequentialOrderRemover<'a>,
}

impl<'a> RandomOrderRemover<'a> {
    pub fn find(&mut self) {}

    pub fn remove(&mut self) {
        // If the outermost is being removed, call SequentialOrderRemover::remove_inner()

        let outermost_removed = false;

        if outermost_removed {
            // Best market price may or may not close
            // Call SequentialOrderRemover::next_active_order()
            // This will clear the current order, move to the next active order
            // and perform market price update
        } else {
            // Remove as usual
            // Best market price does not close
            // Check whether the group closes- group can close only in a non-outermost
            // bitmap group
        }
    }
}
