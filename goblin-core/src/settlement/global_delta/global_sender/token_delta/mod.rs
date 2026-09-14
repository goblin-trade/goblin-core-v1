mod impl_checked_ops;

use goblin_macros::ConstDefault;

use crate::{
    axis::token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    quantities::UnsidedAtoms,
};

#[derive(Clone, Copy, PartialEq, ConstDefault)]
pub struct TokenDelta<TM: TokenQuantity> {
    pub deposit: TM::GlobalDeposit,
    pub take: UnsidedAtoms<i64>,
    pub make: UnsidedAtoms<i64>,
}

impl<TM: TokenMarker> TokenDelta<TM> {
    pub fn net_delta(&self) -> UnsidedAtoms<i64> {
        self.deposit.into() + self.take + self.make
    }
}
