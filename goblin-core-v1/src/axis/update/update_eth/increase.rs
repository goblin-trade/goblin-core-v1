use crate::{
    axis::update::{Increase, UpdateETH},
    goblin_error::GoblinError,
    quantities::ETHAtoms,
    types::Address,
};

impl UpdateETH for Increase {
    fn update_eth(_trader: &Address, _amount: &ETHAtoms) -> Result<(), GoblinError> {
        // Unreachable stub
        Ok(())
    }
}
