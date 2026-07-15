use crate::{
    axis::token::token_marker::TokenMarker,
    quantities::UnsidedDeltaAtoms,
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl<T: TokenMarker> ConstZero for TokenDelta<T> {
    const ZEROED: Self = Self {
        deposit: <T as TokenMarker>::GlobalDeposit::ZEROED,
        take: UnsidedDeltaAtoms::ZEROED,
        make: UnsidedDeltaAtoms::ZEROED,
    };
}
