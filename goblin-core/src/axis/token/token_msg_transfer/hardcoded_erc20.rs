use crate::{
    axis::token::{token_msg_transfer::TokenMsgTransfer, HardcodedERC20Stub},
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtoms,
    settlement::ConstDefault,
};

impl TokenMsgTransfer for HardcodedERC20Stub {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::DEFAULT)
    }

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::DEFAULT)
    }
}
