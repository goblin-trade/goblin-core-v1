use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{CheckedAdd, ConstZero},
};

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

impl CheckedAdd for ETHStub {
    fn checked_add(self, _rhs: Self) -> Option<Self> {
        Some(ETHStub)
    }
}
