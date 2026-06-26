use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{global_delta_v3::DeltaAtomsPair, ConstZero},
};

#[derive(Clone, Copy)]
pub struct TokenDeltaV3<T: TokenMarker> {
    pub deposit: T::GlobalDeposit,
    pub take: DeltaAtomsPair,
    pub make: DeltaAtomsPair,
}

impl<T: TokenMarker> ConstZero for TokenDeltaV3<T> {
    const ZEROED: Self = Self {
        deposit: <T as TokenMarker>::GlobalDeposit::ZEROED,
        take: DeltaAtomsPair::ZEROED,
        make: DeltaAtomsPair::ZEROED,
    };
}
