use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    hostio,
    quantities::{Atoms, RawAtoms},
    settlement::EthDelta,
    types::NATIVE_TOKEN_DECIMALS,
};

pub fn ix_deposit_eth(native_token_delta: &mut EthDelta) -> Result<(), GoblinError> {
    let mut msg_value_maybe = MaybeUninit::<RawAtoms>::uninit();
    let msg_value = unsafe {
        hostio::msg_value(msg_value_maybe.as_mut_ptr() as *mut u8);
        msg_value_maybe.assume_init_ref()
    };

    let atoms = Atoms::from_raw_atoms(msg_value, NATIVE_TOKEN_DECIMALS)?;
    native_token_delta.execute_deposit(atoms)?;

    Ok(())
}
