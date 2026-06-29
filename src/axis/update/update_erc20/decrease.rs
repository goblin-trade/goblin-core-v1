use crate::{
    axis::update::{update_erc20::UpdateERC20, Decrease},
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::RawAtoms,
    types::Address,
};

impl UpdateERC20 for Decrease {
    fn update_erc20<const D: u8>(
        token_address: &Address,
        trader: &Address,
        amount: &RawAtoms<D>,
    ) -> Result<(), GoblinError> {
        erc20_hostio::transfer(token_address, trader, amount)
    }
}
