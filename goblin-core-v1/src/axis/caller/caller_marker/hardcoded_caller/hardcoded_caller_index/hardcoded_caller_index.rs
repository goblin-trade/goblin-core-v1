#[derive(Clone, Copy)]
pub struct HardcodedCallerIndex {
    pub inner: usize,
}

impl HardcodedCallerIndex {
    pub const fn new(inner: usize) -> Self {
        Self { inner }
    }
}
