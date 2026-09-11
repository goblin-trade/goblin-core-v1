use keccak_const::Keccak256;

use crate::state::{Preimage, PreimageSerializer, SlotKey};

/// Trait to get const hash from a preimage
///
/// # Safety
///
/// Implementing structs must be `#[repr(C, packed)]`.
/// Just using `#[repr(C)]` may cause a memory error as the
/// compiler tries to align the fields with uninitialized zeroes.
///
pub const trait ConstPreimage: Preimage {
    fn const_hash(&self) -> SlotKey<Self> {
        let buffer = PreimageSerializer::new(*self);
        let bytes = buffer.serialize();
        let hash = Keccak256::new().update(bytes).finalize();
        SlotKey::<Self>::new(hash)
    }
}
