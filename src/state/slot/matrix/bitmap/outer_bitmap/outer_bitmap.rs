#[repr(C)]
#[derive(PartialEq)]
pub struct OuterBitmap {
    pub inner: [u8; 32],
}

impl OuterBitmap {
    pub const fn new(inner: [u8; 32]) -> Self {
        Self { inner }
    }
}
