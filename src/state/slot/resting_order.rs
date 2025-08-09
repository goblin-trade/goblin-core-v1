use crate::{
    hostio::{self, HostioBuffer},
    quantities::{BaseLots, InnerIndex},
    state::{InnerBitmapKey, SlotKey, SlotState},
    types::Address,
};

pub struct RestingOrderKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKey for RestingOrderKey {
    const DISCRIMINATOR: u8 = 5;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

impl RestingOrderKey {
    pub fn new(inner_bitmap_key: &InnerBitmapKey, inner_index: InnerIndex) -> Self {
        let mut bytes = [0u8; (1 + 32 + 1)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..33].copy_from_slice(inner_bitmap_key.hash());
        bytes[33] = inner_index.0;

        let hash = hostio::native_keccak256(bytes.as_slice());

        Self { hash }
    }
}

/// A resting order stored in slot
/// Total size = 24 + 8 = 32. 20 byte address is padded to 24.
#[repr(C)]
pub struct RestingOrder {
    /// The trader address
    pub trader: Address,
    pub size: BaseLots,
}

impl SlotState<RestingOrderKey> for RestingOrder {}
