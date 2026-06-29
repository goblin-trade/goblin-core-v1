use crate::{
    axis::update::UpdateMarker,
    goblin_error::GoblinError,
    quantities::{RawAtoms, UnsidedAtoms},
    types::Address,
};

pub fn transfer_deposit<UM: UpdateMarker>(
    deposit: UnsidedAtoms,
    decimals: u8,
    token_address: &Address,
    trader: &Address,
) -> Result<(), GoblinError> {
    match decimals {
        6 => {
            let raw_atoms = RawAtoms::<6>::try_from(deposit)?;
            UM::update_erc20(&token_address, &trader, &raw_atoms)
        }
        8 => {
            let raw_atoms = RawAtoms::<8>::try_from(deposit)?;
            UM::update_erc20(&token_address, &trader, &raw_atoms)
        }
        _ => Err(GoblinError::UnsupportedDecimals),
    }
}
