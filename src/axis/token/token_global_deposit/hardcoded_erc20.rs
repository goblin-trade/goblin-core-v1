use crate::{
    axis::token::token_global_deposit::TokenGlobalDeposit, goblin_error::GoblinError,
    quantities::UnsidedDeltaAtoms, settlement::ConstZero,
};

pub struct HardcodedERC20Stub;

impl TokenGlobalDeposit for HardcodedERC20Stub {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::ZEROED)
    }

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::ZEROED)
    }
}
