use crate::{goblin_error::GoblinError, quantities::Delta};

pub trait DeltaAccumulator {
    fn add_consumed_amount(&mut self, consumed: Delta) -> Result<(), GoblinError>;
    fn add_locked_amount(&mut self, locked: Delta) -> Result<(), GoblinError>;
}
