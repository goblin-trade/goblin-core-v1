use crate::{goblin_error::GoblinError, quantities::ETHAtoms, types::Address};

pub trait UpdateETH {
    fn update_eth(trader: &Address, amount: &ETHAtoms) -> Result<(), GoblinError>;
}
