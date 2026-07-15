use crate::{
    axis::token::{token_marker::TokenData, ETH},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::{UnsidedDeltaAtoms, NATIVE_TOKEN_DECIMALS},
    settlement::{CheckedOps, ConstZero},
};
use core::ops::Index;

/// Stub type for ETH token index, address and deposit
///
/// Use an explicit stub type instead of `()` for clarity
#[derive(Default, Clone, Copy, PartialEq)]
pub struct ETHStub;

impl ConstZero for ETHStub {
    const ZEROED: Self = Self;
}

impl Decodable for ETHStub {
    fn try_decode(_ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        Ok(Self)
    }
}

impl CheckedOps for ETHStub {
    fn checked_add(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }

    fn checked_sub(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }
}

impl Into<UnsidedDeltaAtoms> for ETHStub {
    fn into(self) -> UnsidedDeltaAtoms {
        UnsidedDeltaAtoms::ZEROED
    }
}

impl Into<u8> for ETHStub {
    fn into(self) -> u8 {
        NATIVE_TOKEN_DECIMALS
    }
}

impl Index<ETHStub> for ETHStub {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData {
            address: ETHStub,
            decimals: ETHStub,
        }
    }
}
