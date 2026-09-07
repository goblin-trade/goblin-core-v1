use crate::{
    axis::update::UpdateMarker,
    goblin_error::GoblinError,
    input_processor::CallerAddresses,
    quantities::{RawAtoms, UnsidedAtoms},
    types::Address,
};
use core::marker::PhantomData;

pub struct TransferERC20<'a, UM: UpdateMarker> {
    pub token_address: &'a Address,
    pub caller_addresses: CallerAddresses<'a>,
    pub deposit: UnsidedAtoms,
    pub decimals: u8,
    _marker: PhantomData<UM>,
}

impl<'a, UM: UpdateMarker> TransferERC20<'a, UM> {
    pub fn new(
        token_address: &'a Address,
        caller_addresses: CallerAddresses<'a>,
        deposit: UnsidedAtoms,
        decimals: u8,
    ) -> Self {
        Self {
            deposit,
            decimals,
            token_address,
            caller_addresses,
            _marker: PhantomData,
        }
    }

    fn update_erc20_for_decimals<const D: u8>(&self) -> Result<(), GoblinError> {
        let raw_atoms = RawAtoms::<D>::try_from(self.deposit)?;
        let update_address = UM::update_address(self.caller_addresses);
        UM::update_erc20(self.token_address, update_address, &raw_atoms)
    }

    pub fn update_erc20(&self) -> Result<(), GoblinError> {
        match self.decimals {
            6 => self.update_erc20_for_decimals::<6>(),
            8 => self.update_erc20_for_decimals::<8>(),
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
