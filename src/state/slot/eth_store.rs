use crate::{
    hostio::{hostio_native_keccak256, HostioBuffer},
    quantities::Atoms,
    state::{SlotKeyV2, SlotStateV2},
    types::Address,
};

pub struct EthStoreKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKeyV2 for EthStoreKey {
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

        let hash = unsafe { hostio_native_keccak256(bytes.as_slice()) };

        Self { hash }
    }
}

#[repr(C)]
pub struct EthStore {
    pub atoms_locked: Atoms,
    pub atoms_free: Atoms,
    _padding: [u8; 16],
}

impl SlotStateV2<EthStoreKey> for EthStore {}
