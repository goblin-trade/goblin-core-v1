/// The number of dynamic markets to process and their associated custom token addresses
#[derive(Clone, Copy, Default)]
pub struct DynamicCounts {
    pub inner: [u8; 8],
}
