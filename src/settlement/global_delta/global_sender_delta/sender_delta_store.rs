use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{ConstZero, UnsidedSenderDeltaV2},
};

pub struct SenderDeltaStore<T: TokenMarker> {
    /// Atoms to be deposited or withdrawn
    pub deposit_due: T::Deposit,

    /// Delta from trading
    pub unsided_sender_delta: UnsidedSenderDeltaV2,
}

impl<T: TokenMarker> ConstZero for SenderDeltaStore<T> {
    const ZEROED: Self = Self {
        deposit_due: <T as TokenMarker>::Deposit::ZEROED,
        unsided_sender_delta: UnsidedSenderDeltaV2::ZEROED,
    };
}
