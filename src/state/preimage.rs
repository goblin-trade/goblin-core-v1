use crate::{
    hostio::hostio_helpers,
    state::{PreimageSerializer, SlotKey},
};
/// Preimage used to derive slot key. The slot key is then used
/// to read SlotState
pub trait Preimage: Sized {
    /// Unique discriminator for each Preimage implementation
    ///
    /// Discriminators can be standalone or derived from sub-discriminators.
    ///
    /// # Avoiding collisions
    ///
    /// * Standalone: Use 3 bits, i.e. values in [0, 7].
    ///
    /// * Derived discriminator
    ///   - First sub-discriminator takes 3 bits
    ///   - Left shift and add the other ones.
    ///   - Eg. Market discriminator = MarketVariant::D + Base::D << 3 + Quote::D << 4.
    const SLOT_DISCRIMINATOR: u8;

    /// 32 byte slot read using the derived key
    type SlotState;

    /// Hash the preimage to obtain SlotKey
    fn hash(self) -> SlotKey<Self> {
        let buffer = PreimageSerializer::new(self);
        let bytes = buffer.serialize();

        let hash = hostio_helpers::native_keccak256(bytes);
        SlotKey::<Self>::new(hash)
    }
}
