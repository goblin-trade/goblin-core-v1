#[repr(C)]
#[derive(PartialEq)]
pub struct ActiveOuterBitmap {
    pub inner: [u8; 32],
}

impl ActiveOuterBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }
}
