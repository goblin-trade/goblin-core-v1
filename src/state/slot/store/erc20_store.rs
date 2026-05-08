use crate::{impl_checked_slot_state, quantities::UnsidedAtoms, state::Preimage, types::Address};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ERC20StorePreimage {
    trader: Address,
    token: Address,
}

impl Preimage for ERC20StorePreimage {
    const SLOT_DISCRIMINATOR: u8 = 2;
    type SlotState = ERC20Store;
}

#[repr(C)]
pub struct ERC20Store {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: u8,
    _padding: [u8; 15],
}

impl_checked_slot_state!(ERC20Store);
