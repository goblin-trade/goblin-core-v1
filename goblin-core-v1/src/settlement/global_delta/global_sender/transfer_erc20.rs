use crate::{
    axis::update::UpdateMarker,
    goblin_error::GoblinError,
    input_processor::CallerAddresses,
    quantities::{RawAtoms, UnsidedAtoms},
    types::Address,
};
use core::marker::PhantomData;

pub struct TransferERC20<'a, UM: UpdateMarker> {
    pub deposit: UnsidedAtoms,
    pub decimals: u8,
    pub token_address: &'a Address,
    pub caller_addresses: CallerAddresses<'a>,
    _marker: PhantomData<UM>,
}

impl<'a, UM: UpdateMarker> TransferERC20<'a, UM> {
    pub fn new(
        deposit: UnsidedAtoms,
        decimals: u8,
        token_address: &'a Address,
        caller_addresses: CallerAddresses<'a>,
    ) -> Self {
        Self {
            deposit,
            decimals,
            token_address,
            caller_addresses,
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
