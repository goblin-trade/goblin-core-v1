use crate::{
    axis::token::token_global_transfer::TokenGlobalTransfer, goblin_error::GoblinError,
    quantities::UnsidedDeltaAtoms, settlement::ConstZero,
};

#[derive(Clone, Copy)]
pub struct HardcodedERC20Stub;

impl TokenGlobalTransfer for HardcodedERC20Stub {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::ZEROED)
    }

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::ZEROED)
    }
}
