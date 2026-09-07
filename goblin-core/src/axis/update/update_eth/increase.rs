use crate::{
    axis::update::{Increase, UpdateETH},
    goblin_error::GoblinError,
    quantities::ETHAtoms,
    types::Address,
};

impl UpdateETH for Increase {
    fn update_eth(_address: &Address, _amount: &ETHAtoms) -> Result<(), GoblinError> {
        // Unreachable stub. ETH can only be transferred via msg_value at start of
        // call, not during runtime
        Ok(())
    }
}
