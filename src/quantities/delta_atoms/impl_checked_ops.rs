use crate::{quantities::DeltaAtoms, settlement::CheckedOps};

impl CheckedOps for DeltaAtoms {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(DeltaAtoms::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(DeltaAtoms::new)
    }
}
