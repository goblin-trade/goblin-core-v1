use crate::state::Preimage;

/// Efficient serializer for preimages
///
/// Assigns the preimage disciminator at index 0 and returns a serialized byte slice.
/// Serialization is zero-copy and avoids `mut` and zero fills.
///
/// # `packed` representation
///
/// `serialize` exposes the raw bytes of this struct via `from_raw_parts`.
/// With the default `repr(C)` layout the compiler would insert padding
/// between `discriminator` (align 1) and `preimage` (possibly align > 1).
/// Those padding bytes are not initialized, and hashing them during const
/// evaluation is a hard error (`E0080`). `repr(C, packed)` removes the
/// inter-field padding so every byte of the serialized slice is initialized.
///
/// Note this only removes padding *between* the fields; `P` itself must also
/// be padding-free (see `MarketPreimage`).
#[repr(C, packed)]
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

    pub const fn serialize(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                core::mem::size_of::<PreimageSerializer<P>>(),
            )
        }
    }
}
