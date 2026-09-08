mod impl_checked_ops;

use goblin_macros::ConstDefault;

use crate::{
    axis::token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    quantities::UnsidedDeltaAtoms,
};

#[derive(Clone, Copy, PartialEq, ConstDefault)]
pub struct TokenDelta<TM: TokenQuantity> {
    pub deposit: TM::GlobalDeposit,
    pub take: UnsidedDeltaAtoms,
    pub make: UnsidedDeltaAtoms,
}

impl<TM: TokenMarker> TokenDelta<TM> {
    pub fn net_delta(&self) -> UnsidedDeltaAtoms {
        self.deposit.into() + self.take + self.make
    }
}
