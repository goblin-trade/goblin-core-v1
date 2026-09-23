use crate::{
    quantities::{NATIVE_TOKEN_DECIMALS, UnsidedAtoms},
    settlement::ConstDefault,
};

use super::ETHStub;

impl From<ETHStub> for UnsidedAtoms<i64> {
    fn from(_val: ETHStub) -> Self {
        UnsidedAtoms::DEFAULT
    }
}

impl From<ETHStub> for u8 {
    fn from(_val: ETHStub) -> Self {
        NATIVE_TOKEN_DECIMALS
    }
}

impl From<usize> for ETHStub {
    fn from(_: usize) -> Self {
        Self
    }
}
