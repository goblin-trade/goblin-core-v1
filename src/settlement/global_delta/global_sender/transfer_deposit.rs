use crate::{
    axis::{token::token_marker::TokenMarker, update::UpdateMarker},
    goblin_error::GoblinError,
    quantities::{DecimalAction, RawAtoms, UnsidedAtoms},
    types::Address,
};
use core::marker::PhantomData;

pub struct TransferDeposit<'a, T: TokenMarker, UM: UpdateMarker> {
    pub deposit: UnsidedAtoms,
    pub token_address: &'a Address,
    trader: &'a Address,
    _marker: PhantomData<(T, UM)>,
}

impl<'a, T: TokenMarker, UM: UpdateMarker> TransferDeposit<'a, T, UM> {
    pub fn new(deposit: UnsidedAtoms, token_address: &'a Address, trader: &'a Address) -> Self {
        Self {
            deposit,
            token_address,
            trader,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: TokenMarker, UM: UpdateMarker> DecimalAction<T> for TransferDeposit<'a, T, UM> {
    fn run<const D: u8>(&self) -> Result<(), GoblinError> {
        let raw_atoms = RawAtoms::<D>::try_from(self.deposit)?;
        UM::update_erc20(self.token_address, self.trader, &raw_atoms)
    }
}
