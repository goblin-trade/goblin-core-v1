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
        eth_hostio::transfer_out(update_address, &amount)
    }
}
