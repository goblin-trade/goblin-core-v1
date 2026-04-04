#[repr(C)]
#[derive(PartialEq, Default)]
pub struct InnerBitmap {
    pub inner: [u8; 32],
}

impl InnerBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }
}
