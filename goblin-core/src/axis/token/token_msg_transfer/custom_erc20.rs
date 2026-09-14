use crate::{
    axis::token::{CustomERC20Stub, token_msg_transfer::TokenMsgTransfer},
    goblin_error::GoblinError,
    quantities::UnsidedAtoms,
    settlement::ConstDefault,
};

impl TokenMsgTransfer for CustomERC20Stub {
    fn net_delta(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::DEFAULT)
    }

    fn deposit_due(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::DEFAULT)
    }
}

impl TryFrom<CustomERC20Stub> for u8 {
    type Error = GoblinError;

    fn try_from(_value: CustomERC20Stub) -> Result<Self, Self::Error> {
        Err(GoblinError::NoHardcodedDecimals)
    }
}
