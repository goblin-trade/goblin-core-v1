use crate::{
    axis::token::{token_msg_transfer::TokenMsgTransfer, CustomERC20Stub},
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtoms,
    settlement::ConstDefault,
};

impl TokenMsgTransfer for CustomERC20Stub {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::DEFAULT)
    }

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::DEFAULT)
    }
}

impl TryFrom<CustomERC20Stub> for u8 {
    type Error = GoblinError;

    fn try_from(_value: CustomERC20Stub) -> Result<Self, Self::Error> {
        Err(GoblinError::NoHardcodedDecimals)
    }
}
