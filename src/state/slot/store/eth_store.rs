use crate::{impl_checked_slot_state, quantities::UnsidedAtoms, state::Preimage, types::Address};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ERC20StorePreimage {
    trader: Address,
    token: Address,
}

impl Preimage for ERC20StorePreimage {
    const SLOT_DISCRIMINATOR: u8 = 1;
    type SlotState = EthStore;
}

#[repr(C)]
pub struct EthStore {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    _padding: [u8; 16],
}

impl_checked_slot_state!(EthStore);
