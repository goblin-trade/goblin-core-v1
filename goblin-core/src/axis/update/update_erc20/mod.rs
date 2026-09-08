mod decrease;
mod increase;

use crate::{goblin_error::GoblinError, quantities::RawAtoms, types::Address};

pub trait UpdateERC20 {
    fn update_erc20<const D: u8>(
        token_address: &Address,
        trader: &Address,
        amount: &RawAtoms<D>,
    ) -> Result<(), GoblinError>;
}
