use crate::state::Preimage;

#[repr(C)]
pub struct PreimageSerializer<P: Preimage> {
    discriminator: u8,
    preimage: P,
}

impl<P: Preimage> PreimageSerializer<P> {
    pub fn new(preimage: P) -> Self {
        Self {
            discriminator: P::SLOT_DISCRIMINATOR,
            preimage,
        }
    }

    pub fn serialize(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                core::mem::size_of::<Self<P>>(),
            )
        }
    }
}
