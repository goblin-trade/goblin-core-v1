use crate::state::{SlotKey, SlotState};

#[repr(C)]
pub struct InnerBitmap(pub [u8; 32]);

impl SlotState for InnerBitmap {
    const SLOT_DISCRIMINATOR: u8 = 6;
}

impl SlotKey<InnerBitmap> {}

// impl InnerBitmapKey {
//     pub fn new(market_key: &MarketKey, inner_bitmap_index: InnerBitmapIndex) -> Self {
//         let mut bytes = [0u8; (1 + 32 + 4)];
//         bytes[0] = Self::DISCRIMINATOR;
//         bytes[1..33].copy_from_slice(market_key.hash());
//         bytes[33..37].copy_from_slice(&inner_bitmap_index.0.to_le_bytes());

//         let hash = hostio::native_keccak256(bytes.as_slice());

//         Self { hash }
//     }
// }
