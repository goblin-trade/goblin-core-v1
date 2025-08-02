use crate::{
    hostio::{self, HostioBuffer},
    quantities::Atoms,
    state::{SlotKey, SlotState},
    types::Address,
};

pub struct EthStoreKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKey for EthStoreKey {
    const DISCRIMINATOR: u8 = 1;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

impl EthStoreKey {
    pub fn new(trader: &Address) -> Self {
        let mut bytes = [0u8; (1 + 20)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..21].copy_from_slice(trader.as_slice());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Self { hash }
    }
}

#[repr(C)]
pub struct EthStore {
    pub atoms_locked: Atoms,
    pub atoms_free: Atoms,
    _padding: [u8; 16],
}

impl SlotState<EthStoreKey> for EthStore {}
