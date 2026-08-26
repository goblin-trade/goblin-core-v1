use crate::settlement::{global_delta::GlobalCounterparty, CheckedOps};

impl CheckedOps for GlobalCounterparty {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            inner: self.inner.checked_add(rhs.inner)?,
        })
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            inner: self.inner.checked_sub(rhs.inner)?,
        })
    }
}
