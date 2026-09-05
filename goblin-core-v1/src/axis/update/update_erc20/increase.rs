use crate::{
    axis::update::{update_erc20::UpdateERC20, Increase},
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::RawAtoms,
    types::Address,
    user_entrypoint::CONTRACT_ADDRESS,
};

impl UpdateERC20 for Increase {
    fn update_erc20<const D: u8>(
        token_address: &Address,
        trader: &Address,
        amount: &RawAtoms<D>,
    ) -> Result<(), GoblinError> {
        erc20_hostio::transfer_from(token_address, trader, &CONTRACT_ADDRESS, amount)
    }
}
