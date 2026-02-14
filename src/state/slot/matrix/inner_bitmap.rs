use crate::{
    axis::{market::market_marker::MarketMarker, token::token_marker::TokenMarker},
    matching::bitmap::OuterPos,
    state::{outer_bitmap::OuterBitmapPreimage, Preimage, SlotKey},
};

#[repr(C)]
pub struct InnerBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q>>,
    outer_pos: OuterPos,
}

impl<M, B, Q> Preimage for InnerBitmapPreimage<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    const SLOT_DISCRIMINATOR: u8 = 6;
    type SlotState = InnerBitmap;
}

#[repr(C)]
pub struct InnerBitmap(pub [u8; 32]);

// impl SlotState for InnerBitmap {
//     const SLOT_DISCRIMINATOR: u8 = 6;
// }

// impl SlotKey<InnerBitmap> {}

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
