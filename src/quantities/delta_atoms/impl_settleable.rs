use crate::{
    goblin_error::GoblinError, hostio::erc20_hostio, processor::CONTRACT_ADDRESS,
    quantities::DeltaAtoms, settlement::Settleable, types::Address,
};

impl Settleable for DeltaAtoms {
    fn settle(&self, token_address: &Address, msg_sender: &Address) -> Result<(), GoblinError> {
        if self.inner > 0 {
            erc20_hostio::transfer_from(token_address, msg_sender, &CONTRACT_ADDRESS, amount)?;
        }

        Ok(())
    }
}
