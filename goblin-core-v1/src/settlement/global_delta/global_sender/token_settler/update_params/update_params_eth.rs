use crate::{
    axis::{UpdateMarker, ETH},
    goblin_error::GoblinError,
    hostio::eth_hostio,
    quantities::ETHAtoms,
    settlement::UpdateParams,
};

impl<'a, UM: UpdateMarker> UpdateParams<'a, ETH, UM> {
    pub fn update_eth(&self) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(self.deposit)?;
        let update_address = UM::get_update_address(self.caller_addresses);

        // error found- passing amount causes call to fail
        eth_hostio::transfer_out(update_address, &amount)
        // eth_hostio::transfer_out(update_address, &ETHAtoms::default())

        // let amount = ETHAtoms::try_from(self.deposit)?;
        // // let update_address = UM::get_update_address(self.caller_addresses);

        // // let update_address = self.caller_addresses.caller;

        // // this causes problem
        // eth_hostio::transfer_out(&[0u8; 20], &amount)
        // eth_hostio::transfer_out(update_address, &amount)
        // Ok(())
        // UM::update_eth(update_address, &amount)
    }
}
