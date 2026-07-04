use crate::{goblin_error::GoblinError, quantities::UnsidedDeltaAtoms};

pub trait TokenGlobalDeposit {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError>;

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError>;
}
