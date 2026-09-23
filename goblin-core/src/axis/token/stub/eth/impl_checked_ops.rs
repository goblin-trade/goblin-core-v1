use crate::settlement::CheckedOps;

use super::ETHStub;

impl CheckedOps for ETHStub {
    fn checked_add(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }

    fn checked_sub(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }

    fn checked_mul(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }
}
