use goblin_macros::ConstDefault;

use crate::{
    axis::token::{token_marker::TokenData, ETH},
    input_processor::{ArgsReader, FixedDecode},
    quantities::{UnsidedDeltaAtoms, NATIVE_TOKEN_DECIMALS},
    settlement::{CheckedOps, ConstDefault},
};
use core::ops::Index;

/// Stub type for ETH token index, address and deposit
///
/// Use an explicit stub type instead of `()` for clarity
#[derive(Default, Clone, Copy, PartialEq, ConstDefault)]
pub struct ETHStub;

impl<'a> FixedDecode<'a> for ETHStub {
    const ENCODED_SIZE: usize = 0;

    fn raw_fixed_decode(_reader: &'a ArgsReader) -> Self {
        Self
    }
}

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

impl Into<UnsidedDeltaAtoms> for ETHStub {
    fn into(self) -> UnsidedDeltaAtoms {
        UnsidedDeltaAtoms::DEFAULT
    }
}

impl Into<u8> for ETHStub {
    fn into(self) -> u8 {
        NATIVE_TOKEN_DECIMALS
    }
}

impl From<usize> for ETHStub {
    fn from(_: usize) -> Self {
        Self
    }
}

impl Index<ETHStub> for ETHStub {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData::DEFAULT
    }
}

impl<'a> IntoIterator for &'a TokenData<ETH> {
    type Item = &'a TokenData<ETH>;
    type IntoIter = core::iter::Once<&'a TokenData<ETH>>;

    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(self)
    }
}
