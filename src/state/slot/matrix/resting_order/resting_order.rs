use crate::{quantities::BaseLots, types::Address};

/// A resting order stored in slot
/// Total size = 24 + 8 = 32. 20 byte address is padded to 24.
///
/// # Definition
///
/// * A resting order to buy or sell the given number of base lots at the given price.
///
/// * For base in (Ask)- the maker locks in `size: BaseLots` and expects to get
/// `quote lots = size * price`
///
/// * For quote in (Bid)- the maker locks in `quote lots = size * price` and expects
/// to get `size: BaseLots`
///
/// # Intermediary units for taker
///
/// * As the stored quantity is base lots, the matching unit for base in (ask) is BaseLots.
/// * If quote in case (bid), we use adjustedQuoteLots = quote lots * BaseLotsPerBaseUnit
/// * Base in taker (ask) is matched against quote in maker (bid).
#[repr(C)]
pub struct RestingOrder {
    pub maker: Address,
    pub size: BaseLots,
}

impl RestingOrder {}

// impl SlotState for RestingOrder {
//     const SLOT_DISCRIMINATOR: u8 = 7;
// }

// impl SlotKey<RestingOrder> {
//     pub fn new(inner_bitmap_key: &SlotKey<InnerBitmap>, inner_index: InnerIndex) -> Self {
//         let mut bytes = [0u8; (1 + 32 + 1)];
//         bytes[0] = RestingOrder::SLOT_DISCRIMINATOR;
//         bytes[1..33].copy_from_slice(inner_bitmap_key.hash());
//         bytes[33] = inner_index.0;

//         Self::generate(bytes.as_slice())
//     }
// }
