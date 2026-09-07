use crate::{
    axis::{UpdateMarker, ETH},
    goblin_error::GoblinError,
    quantities::ETHAtoms,
    settlement::UpdateParams,
};

impl<'a, UM: UpdateMarker> UpdateParams<'a, ETH, UM> {
    pub fn update_eth(&self) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(self.deposit)?;
        let update_address = UM::get_update_address(self.caller_addresses);
        UM::update_eth(update_address, &amount)
    }
}
