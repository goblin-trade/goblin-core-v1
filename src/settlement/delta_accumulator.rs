use crate::{goblin_error::GoblinError, quantities::AtomsDelta};

pub trait DeltaAccumulator {
    fn add_consumed_amount(&mut self, consumed: AtomsDelta) -> Result<(), GoblinError>;
    fn add_locked_amount(&mut self, locked: AtomsDelta) -> Result<(), GoblinError>;
}
