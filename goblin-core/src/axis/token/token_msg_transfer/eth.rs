use crate::{
    axis::TokenMsgTransfer, goblin_error::GoblinError, input_processor::ETHTransfers,
    quantities::UnsidedDeltaAtoms,
};

impl TokenMsgTransfer for ETHTransfers {
    fn net_delta(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        Ok(UnsidedDeltaAtoms::try_from(self.msg_value)?
            - UnsidedDeltaAtoms::try_from(self.eth_out_due)?)
    }

    fn deposit_due(&self) -> Result<UnsidedDeltaAtoms, GoblinError> {
        // Reverse the sign to convert out_due to deposit_due
        Ok(-UnsidedDeltaAtoms::try_from(self.eth_out_due)?)
    }
}
