use crate::{quantities::UnsidedAtoms, state::SlotKey, types::Address};

#[repr(C)]
pub struct ERC20Store {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: u8,
    _padding: [u8; 15],
}

impl SlotKey<ERC20Store, 2> {
    pub fn new(trader: &Address, token: &Address) -> Self {
        let mut bytes = [0u8; (1 + 20 + 20)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..21].copy_from_slice(trader.as_slice());
        bytes[21..41].copy_from_slice(token.as_slice());

        Self::generate(bytes.as_slice())
    }
}

// impl SlotState for ERC20Store {
//     const DISCRIMINATOR: u8 = 2;
// }

// pub struct ERC20StoreKey {
//     hash: HostioBuffer<[u8; 32]>,
// }

// impl SlotKey for ERC20StoreKey {
//     const DISCRIMINATOR: u8 = 2;

//     fn hash(&self) -> &[u8; 32] {
//         self.hash.as_ref()
//     }
// }

// impl ERC20StoreKey {
//     pub fn new(trader: &Address, token: &Address) -> Self {
//         let mut bytes = [0u8; (1 + 20 + 20)];
//         bytes[0] = Self::DISCRIMINATOR;
//         bytes[1..21].copy_from_slice(trader.as_slice());
//         bytes[21..41].copy_from_slice(token.as_slice());

//         let hash = hostio::native_keccak256(bytes.as_slice());

//         Self { hash }
//     }
// }

// #[repr(C)]
// pub struct ERC20Store {
//     pub atoms_locked: UnsidedAtoms,
//     pub atoms_free: UnsidedAtoms,
//     pub decimals: u8,
//     _padding: [u8; 15],
// }

// impl SlotState<ERC20StoreKey> for ERC20Store {}

// impl ERC20Store {
//     pub fn is_empty(&self) -> bool {
//         unsafe {
//             let words = &*(self as *const ERC20Store as *const [u64; 4]);
//             (words[0] | words[1] | words[2] | words[3]) == 0
//         }
//     }
// }
