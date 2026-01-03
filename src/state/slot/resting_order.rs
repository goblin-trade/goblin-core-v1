use crate::{
    hostio::{self, HostioBuffer},
    quantities::{BaseLots, InnerIndex},
    state::{InnerBitmap, SlotKey, SlotState},
    types::Address,
};

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
    /// The maker address
    pub maker: Address,
    pub size: BaseLots,
}

impl SlotState for RestingOrder {
    const DISCRIMINATOR: u8 = 5;
}

impl SlotKey<RestingOrder> {
    pub fn new(inner_bitmap_key: &SlotKey<InnerBitmap>, inner_index: InnerIndex) -> Self {
        let mut bytes = [0u8; (1 + 32 + 1)];
        bytes[0] = RestingOrder::DISCRIMINATOR;
        bytes[1..33].copy_from_slice(inner_bitmap_key.hash());
        bytes[33] = inner_index.0;

        Self::generate(bytes.as_slice())
    }
}

// pub struct RestingOrderKey {
//     hash: HostioBuffer<[u8; 32]>,
// }

// impl SlotKey for RestingOrderKey {
//     const DISCRIMINATOR: u8 = 5;

//     fn hash(&self) -> &[u8; 32] {
//         self.hash.as_ref()
//     }
// }

// impl RestingOrderKey {
//     pub fn new(inner_bitmap_key: &InnerBitmapKey, inner_index: InnerIndex) -> Self {
//         let mut bytes = [0u8; (1 + 32 + 1)];
//         bytes[0] = Self::DISCRIMINATOR;
//         bytes[1..33].copy_from_slice(inner_bitmap_key.hash());
//         bytes[33] = inner_index.0;

//         let hash = hostio::native_keccak256(bytes.as_slice());

//         Self { hash }
//     }
// }

// /// A resting order stored in slot
// /// Total size = 24 + 8 = 32. 20 byte address is padded to 24.
// ///
// /// # Definition
// ///
// /// * A resting order to buy or sell the given number of base lots at the given price.
// ///
// /// * For base in (Ask)- the maker locks in `size: BaseLots` and expects to get
// /// `quote lots = size * price`
// ///
// /// * For quote in (Bid)- the maker locks in `quote lots = size * price` and expects
// /// to get `size: BaseLots`
// ///
// /// # Intermediary units for taker
// ///
// /// * As the stored quantity is base lots, the matching unit for base in (ask) is BaseLots.
// /// * If quote in case (bid), we use adjustedQuoteLots = quote lots * BaseLotsPerBaseUnit
// /// * Base in taker (ask) is matched against quote in maker (bid).
// #[repr(C)]
// pub struct RestingOrder {
//     /// The maker address
//     pub maker: Address,
//     pub size: BaseLots,
// }

// impl SlotState<RestingOrderKey> for RestingOrder {}
