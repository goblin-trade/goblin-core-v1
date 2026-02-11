/// The number of hardcoded markets to process
#[derive(Clone, Copy)]
pub struct HardcodedCounts {
    pub inner: [u8; 3],
}

impl HardcodedCounts {
    pub fn new(inner: [u8; 3]) -> Self {
        Self { inner }
    }
}
