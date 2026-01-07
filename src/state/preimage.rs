use crate::{
    hostio::hostio_helpers,
    state::{PreimageSerializer, SlotKey},
};

pub trait Preimage: Sized {
    const SLOT_DISCRIMINATOR: u8;

    type SlotState;

    fn generate(self) -> SlotKey<Self> {
        let buffer = PreimageSerializer::new(self);
        let bytes = buffer.serialize();

        let hash = hostio_helpers::native_keccak256(bytes);
        SlotKey::<Self>::new_inner(hash)
    }
}
