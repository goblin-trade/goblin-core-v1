use crate::{
    axis::token::token_reader::TokenReader,
    settlement::{ConstZero, UnsidedSenderDeltaV2},
};

#[derive(Clone, Copy)]
pub struct SenderTokenStore<T: TokenReader> {
    /// Atoms to be deposited or withdrawn
    pub deposit_due: T::Deposit,

    /// Delta from trading
    pub unsided_sender_delta: UnsidedSenderDeltaV2,
}

impl<T: TokenReader> ConstZero for SenderTokenStore<T> {
    const ZEROED: Self = Self {
        deposit_due: <T as TokenReader>::Deposit::ZEROED,
        unsided_sender_delta: UnsidedSenderDeltaV2::ZEROED,
    };
}
