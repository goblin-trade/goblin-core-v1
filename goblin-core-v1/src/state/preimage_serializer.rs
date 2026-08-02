use crate::state::Preimage;

/// Efficient serializer for preimages
///
/// Assigns the preimage disciminator at index 0 and returns a serialized byte slice.
/// Serialization is zero-copy and avoids `mut` and zero fills.
#[repr(C)]
pub struct PreimageSerializer<P: Preimage> {
    discriminator: u8,
    preimage: P,
}

impl<P: Preimage> PreimageSerializer<P> {
    pub const fn new(preimage: P) -> Self {
        Self {
            discriminator: P::SLOT_DISCRIMINATOR,
            preimage,
        }
    }

    pub fn serialize(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                core::mem::size_of::<PreimageSerializer<P>>(),
            )
        }
    }
}
