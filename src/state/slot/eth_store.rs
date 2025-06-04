use crate::{
    hostio::{hostio_native_keccak256, HostioBuffer},
    quantities::Atoms,
    state::SlotKeyV2,
    types::Address,
};

pub struct EthStoreKey {
    pub trader: Address,
}

impl SlotKeyV2<EthStore> for EthStoreKey {
    const DISCRIMINATOR: u8 = 1;

    fn to_keccak256(&self) -> HostioBuffer<[u8; 32]> {
        const BYTE_SIZE: usize = core::mem::size_of::<EthStoreKey>();

        let bytes = {
            let mut b = [0u8; (1 + BYTE_SIZE)];
            b[0] = Self::DISCRIMINATOR;

            let self_slice =
                unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, BYTE_SIZE) };
            b[1..].copy_from_slice(self_slice);

            b
        };

        unsafe { hostio_native_keccak256(bytes.as_slice()) }
    }
}

#[repr(C)]
pub struct EthStore {
    pub atoms_locked: Atoms,
    pub atoms_free: Atoms,
    _padding: [u8; 16],
}

// impl HostioBuffer<EthStore> {
//     pub fn init(key: &EthStoreKey) -> Self {
//         let hashed_key = key.to_keccak256();
//         unsafe { hostio_storage_load_bytes32::<EthStore>(hashed_key.as_ref()) }
//     }
// }
