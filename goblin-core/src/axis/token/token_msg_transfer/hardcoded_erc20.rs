use crate::{
    axis::token::{HardcodedERC20Stub, token_msg_transfer::TokenMsgTransfer},
    goblin_error::GoblinError,
    quantities::UnsidedAtoms,
    settlement::ConstDefault,
};

impl TokenMsgTransfer for HardcodedERC20Stub {
    fn net_delta(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::DEFAULT)
    }

    fn deposit_due(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::DEFAULT)
    }
}
