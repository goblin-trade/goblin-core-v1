#[derive(Default, Clone, Copy, PartialEq)]
pub struct DeltaAtoms {
    inner: i64,
}

impl DeltaAtoms {
    pub const ZERO: Self = DeltaAtoms { inner: 0 };

    pub fn new(inner: i64) -> Self {
        Self { inner }
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(DeltaAtoms::new)
    }
}
