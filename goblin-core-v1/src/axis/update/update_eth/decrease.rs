use crate::{
    axis::update::{Decrease, UpdateETH},
    goblin_error::GoblinError,
    hostio::eth_hostio,
    quantities::ETHAtoms,
    types::Address,
};

impl UpdateETH for Decrease {
    fn update_eth(trader: &Address, amount: &ETHAtoms) -> Result<(), GoblinError> {
        eth_hostio::transfer_out(trader, amount)
    }
}
