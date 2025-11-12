use crate::{
    hostio::{self, HostioBuffer},
    quantities::OuterBitmapIndex,
    state::{SlotKey, SlotState},
};

pub struct OuterBitmapKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKey for OuterBitmapKey {
    const DISCRIMINATOR: u8 = 6;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

// impl OuterBitmapKey {
//     pub fn new(market_key: &MarketKey, outer_bitmap_index: OuterBitmapIndex) -> Self {
//         let mut bytes = [0u8; (1 + 32 + 4)];
//         bytes[0] = Self::DISCRIMINATOR;
//         bytes[1..33].copy_from_slice(market_key.hash());
//         bytes[33..37].copy_from_slice(&outer_bitmap_index.0.to_le_bytes());

//         let hash = hostio::native_keccak256(bytes.as_slice());

//         Self { hash }
//     }
// }

#[repr(C)]
pub struct OuterBitmap(pub [u8; 32]);

impl SlotState<OuterBitmapKey> for OuterBitmap {}
