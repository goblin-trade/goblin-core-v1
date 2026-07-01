use crate::{
    axis::update::UpdateMarker,
    goblin_error::GoblinError,
    quantities::{DecimalAction, RawAtoms, UnsidedAtoms},
    types::Address,
};
use core::marker::PhantomData;

pub struct TransferDeposit<'a, UM: UpdateMarker> {
    pub deposit: UnsidedAtoms,
    pub token_address: &'a Address,
    trader: &'a Address,
    _marker: PhantomData<UM>,
}

impl<'a, UM: UpdateMarker> TransferDeposit<'a, UM> {
    pub fn new(deposit: UnsidedAtoms, token_address: &'a Address, trader: &'a Address) -> Self {
        Self {
            deposit,
            token_address,
            trader,
            _marker: PhantomData,
        }
    }
}

impl<'a, UM: UpdateMarker> DecimalAction for TransferDeposit<'a, UM> {
    fn run<const D: u8>(&self) -> Result<(), GoblinError> {
        let raw_atoms = RawAtoms::<D>::try_from(self.deposit)?;
        UM::update_erc20(self.token_address, self.trader, &raw_atoms)
    }
}
