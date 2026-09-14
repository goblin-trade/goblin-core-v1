use crate::{
    axis::token::TokenMsgTransfer, goblin_error::GoblinError, input_processor::ETHTransfers,
    quantities::UnsidedAtoms,
};

impl TokenMsgTransfer for ETHTransfers {
    fn net_delta(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        Ok(UnsidedAtoms::<i64>::try_from(self.msg_value)?
            - UnsidedAtoms::<i64>::try_from(self.eth_out_due)?)
    }

    fn deposit_due(&self) -> Result<UnsidedAtoms<i64>, GoblinError> {
        // Reverse the sign to convert out_due to deposit_due
        Ok(-UnsidedAtoms::<i64>::try_from(self.eth_out_due)?)
    }
}
