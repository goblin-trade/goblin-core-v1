use crate::{
    axis::{TokenMarker, UpdateMarker},
    goblin_error::GoblinError,
    quantities::RawAtoms,
    settlement::UpdateParams,
    types::Address,
};

impl<'a, TM, UM> UpdateParams<'a, TM, UM>
where
    TM: TokenMarker<TokenAddress = Address, StoredDecimals = u8>,
    UM: UpdateMarker,
{
    pub fn update_erc20(&self) -> Result<(), GoblinError> {
        match self.decimals {
            6 => self.update_erc20_for_decimals::<6>(),
            8 => self.update_erc20_for_decimals::<8>(),
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }

    fn update_erc20_for_decimals<const D: u8>(&self) -> Result<(), GoblinError> {
        let raw_atoms = RawAtoms::<D>::try_from(self.deposit)?;
        let update_address = UM::get_update_address(self.caller_addresses);
        UM::update_erc20(self.token_address, update_address, &raw_atoms)
    }
}
