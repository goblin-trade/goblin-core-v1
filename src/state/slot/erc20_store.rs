use crate::{quantities::UnsidedAtoms, state::Preimage, types::Address};

#[repr(C)]
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

// impl SlotKey<ERC20StorePreimage, ERC20Store> {
//     pub fn new(trader: &Address, token: &Address) -> Self {
//         let mut bytes = [0u8; (1 + 20 + 20)];
//         bytes[0] = ERC20Store::SLOT_DISCRIMINATOR;
//         bytes[1..21].copy_from_slice(trader.as_slice());
//         bytes[21..41].copy_from_slice(token.as_slice());

//         Self::generate(bytes.as_slice())
//     }
// }

// impl ERC20Store {
//     pub fn is_empty(&self) -> bool {
//         unsafe {
//             let words = &*(self as *const ERC20Store as *const [u64; 4]);
//             (words[0] | words[1] | words[2] | words[3]) == 0
//         }
//     }
// }
