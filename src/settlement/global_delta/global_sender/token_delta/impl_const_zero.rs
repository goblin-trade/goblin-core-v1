use crate::{
    axis::token::token_marker::TokenMarker,
    quantities::DeltaAtoms,
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl<T: TokenMarker> ConstZero for TokenDelta<T> {
    const ZEROED: Self = Self {
        deposit: <T as TokenMarker>::GlobalDeposit::ZEROED,
        take: DeltaAtoms::ZEROED,
        make: DeltaAtoms::ZEROED,
    };
}
