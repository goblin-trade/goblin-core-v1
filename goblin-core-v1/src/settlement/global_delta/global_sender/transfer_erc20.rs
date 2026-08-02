use crate::{
    axis::update::UpdateMarker,
    goblin_error::GoblinError,
    quantities::{RawAtoms, UnsidedAtoms},
    types::Address,
};
use core::marker::PhantomData;

pub struct TransferERC20<'a, UM: UpdateMarker> {
    pub deposit: UnsidedAtoms,
    pub trader: &'a Address,
    pub token_address: &'a Address,
    pub decimals: u8,
    _marker: PhantomData<UM>,
}

impl<'a, UM: UpdateMarker> TransferERC20<'a, UM> {
    pub fn new(
        deposit: UnsidedAtoms,
        trader: &'a Address,
        token_address: &'a Address,
        decimals: u8,
    ) -> Self {
        Self {
            deposit,
            decimals,
            trader,
            token_address,
            _marker: PhantomData,
        }
    }

    fn run<const D: u8>(&self) -> Result<(), GoblinError> {
        let raw_atoms = RawAtoms::<D>::try_from(self.deposit)?;
        UM::update_erc20(self.token_address, self.trader, &raw_atoms)
    }

    pub fn dispatch(&self) -> Result<(), GoblinError> {
        match self.decimals {
            6 => self.run::<6>(),
            8 => self.run::<8>(),
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
