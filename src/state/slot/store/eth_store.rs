use crate::{quantities::UnsidedAtoms, state::Preimage, types::Address};

#[repr(C)]
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

// impl SlotState for EthStore {
//     const SLOT_DISCRIMINATOR: u8 = 1;
// }

// impl SlotKey<EthStore> {
//     pub fn new(trader: &Address) -> Self {
//         let mut bytes = [0u8; (1 + 20)];
//         bytes[0] = EthStore::SLOT_DISCRIMINATOR;
//         bytes[1..21].copy_from_slice(trader.as_slice());

//         Self::generate(bytes.as_slice())
//     }
// }
