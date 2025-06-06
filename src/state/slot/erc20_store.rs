use crate::{
    hostio::{hostio_native_keccak256, HostioBuffer},
    quantities::Atoms,
    state::{SlotKeyV2, SlotStateV2},
    types::Address,
};

pub struct Erc20StoreKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKeyV2 for Erc20StoreKey {
    const DISCRIMINATOR: u8 = 2;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

impl Erc20StoreKey {
    pub fn new(trader: &Address, token: &Address) -> Self {
        let mut bytes = [0u8; (1 + 20 + 20)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..21].copy_from_slice(trader.as_slice());
        bytes[21..41].copy_from_slice(token.as_slice());

        let hash = unsafe { hostio_native_keccak256(bytes.as_slice()) };

        Self { hash }
    }
}

#[repr(C)]
pub struct Erc20Store {
    pub atoms_locked: Atoms,
    pub atoms_free: Atoms,
    pub decimals: u8,
    _padding: [u8; 15],
}

impl SlotStateV2<Erc20StoreKey> for Erc20Store {}
