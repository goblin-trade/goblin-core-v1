mod decrease;
mod increase;

use crate::{goblin_error::GoblinError, quantities::ETHAtoms, types::Address};

pub trait UpdateETH {
    fn update_eth(address: &Address, amount: &ETHAtoms) -> Result<(), GoblinError>;
}
