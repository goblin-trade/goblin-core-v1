use crate::{
    axis::{TokenMarker, UpdateEnum},
    goblin_error::GoblinError,
    input_processor::{caller_addresses, CallerAddresses},
    match_axes,
    quantities::{IntoAbs, UnsidedDeltaAtoms},
    types::Address,
};

pub fn transfer_token<TM: TokenMarker>(
    net_deposit: UnsidedDeltaAtoms,
    token_address: &TM::TokenAddress,
    decimals: TM::StoredDecimals,
    caller_addresses: CallerAddresses,
) -> Result<(), GoblinError> {
    if net_deposit == UnsidedDeltaAtoms::default() {
        return Ok(());
    }

    let update_enum = UpdateEnum::from(net_deposit);
    let deposit = net_deposit.abs();

    match_axes!(UM = update_enum => {
        TM::update::<UM>(deposit, trader, token_address, decimals)?;
    });

    Ok(())
}
