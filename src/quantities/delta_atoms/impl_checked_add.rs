use crate::{quantities::DeltaAtoms, settlement::CheckedAdd};

impl CheckedAdd for DeltaAtoms {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(DeltaAtoms::new)
    }
}
